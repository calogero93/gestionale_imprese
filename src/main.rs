use async_redis_session::RedisSessionStore;
use axum::{
    extract::{Json, Query, State}, response::{IntoResponse, Response}, routing::{get, post}, Router
};
use diesel::{prelude::*, sql_query, sql_types::{Nullable, Text}};
use diesel::r2d2::{self, ConnectionManager};
use hyper::StatusCode;
use redis::Client;
use schema::utentis::impresa_id;
use subschemas::employees;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task;
use dotenvy::dotenv;
use std::env;
use axum_sessions::{extractors::{ReadableSession, WritableSession}, SessionLayer};

mod schema;
mod models;
mod migration;
mod utils;
mod subschemas;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct UserRequest {
    user_id: i32,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
    impresa_id: i32
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}

#[derive(Deserialize)]
struct AddEmployeeRequest {
    nome: String,
    cognome: String,
    ruolo: String,
}

#[derive(Deserialize)]
struct GetUserDataQuery {
    nome: Option<String>,
    cognome: Option<String>,
    ruolo: Option<String>,
}

#[derive(Deserialize)]
struct GetEmployeeDataQuery {
    id: i32,
}

#[derive(Deserialize)]
struct UpdateEmployeeRequest {
    id: i32,
    nome: Option<String>,
    cognome: Option<String>,
    ruolo: Option<String>,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[diesel(table_name = employees)]
struct UpdateEmployeeFields {
    nome: Option<String>,
    cognome: Option<String>,
    ruolo: Option<String>,
}

#[derive(Deserialize)]
struct RemoveEmployeeQuery {
    id: i32,
}

pub struct InternalServerError(pub anyhow::Error);

impl IntoResponse for InternalServerError {
fn into_response(self) -> Response {
(StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP 500: {}", self.0)).into_response()
}
}

// This enables using ? on functions that return Result<_, anyhow::Error> to turn them into
// Result<_, AppError>. That way you don't need to do that manually.
impl<E> From<E> for InternalServerError
where
E: Into<anyhow::Error>,
{
fn from(err: E) -> Self {
Self(err.into())
}
}




async fn register(
    State(pool): State<Arc<DbPool>>,
    payload: Json<RegisterRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::users;

    let new_user = models::NewUser {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    //utils::set_search_path(&mut conn, "public").map_err(|e| format!("Failed to set search path: {}", e))?;

    let user_id: i32 = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(users::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert user: {}", e))?;

    let schema_name = format!("user_{}", user_id);

    diesel::sql_query(format!("CREATE SCHEMA {}", schema_name))
        .execute(&mut conn)
        .map_err(|e| format!("Failed to create schema: {}", e))?;

    // Eseguire le migrazioni per lo schema specifico
    task::block_in_place(|| migration::run(&schema_name, &mut conn))
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    //utils::set_search_path(&mut conn, &schema_name).map_err(|e| format!("Failed to set search path: {}", e))?;

    diesel::sql_query(format!(
        "CREATE TABLE {}.employees (
            id SERIAL PRIMARY KEY,
            nome VARCHAR(100) NOT NULL,
            cognome VARCHAR(100) NOT NULL,
            ruolo VARCHAR(50) NOT NULL
        )",
        schema_name
    ))
    .execute(&mut conn)
    .map_err(|e| format!("Failed to create dipendenti table: {}", e))?;


    Ok(Json(format!("User {} registered with schema {}", payload.username, schema_name)).into_response())
    
}

async fn get_user_data(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetUserDataQuery>,
) -> Result<Json<Vec<models::Dipendenti>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    println!("{:?}", &session.get::<String>("user_id"));

    let schema_name = format!("user_{}", user_id);
    let table_name = format!("{}.employees", schema_name);

    let mut query = format!("SELECT * FROM {}", table_name);
    let mut conditions = vec![];

    if let Some(nome) = &params.nome {
        conditions.push(format!("nome LIKE '%{}%'", nome));
    }
    if let Some(cognome) = &params.cognome {
        conditions.push(format!("cognome LIKE '%{}%'", cognome));
    }
    if let Some(ruolo) = &params.ruolo {
        conditions.push(format!("ruolo LIKE '%{}%'", ruolo));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }


    let results = sql_query(query)
        .load::<models::Dipendenti>(&mut conn)
        .map_err(|e| format!("Failed to execute query: {}", e))?;


    Ok(Json(results))
}

async fn login(
    mut session: WritableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;
    use schema::utentis::dsl::*;

    println!("{} {}", &payload.username, &payload.password);

   // utils::set_search_path(&mut conn, "public").map_err(|e| format!("Failed to set search path: {}", e))?;

    let user = utentis
        .filter(username.eq(&payload.username))
        .filter(password.eq(&payload.password))
        .filter(impresa_id.eq(&payload.impresa_id))
        .first::<models::Utenti>(&mut conn)
        .map_err(|_| "Invalid username or password".to_string())?;

    session.insert("user_id", user.id.to_string()).unwrap();

    
    Ok(Json(LoginResponse { message: "Login successful".to_string() }))
}

async fn add_employee(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Json(payload): Json<AddEmployeeRequest>,
) -> Result<impl IntoResponse, String> {
    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let schema_name = format!("user_{}", user_id);
    let table_name = format!("{}.employees", schema_name);

    // Costruzione della query SQL con parametri
    let query = format!(
        r#"INSERT INTO "{}"."employees" (nome, cognome, ruolo) VALUES ($1, $2, $3)"#,
        schema_name
    );
    //utils::set_search_path(&mut conn, &schema_name).map_err(|e| format!("Failed to set search path: {}", e))?;

    /*let new_dipendente = models::NewEmployee {
        nome: payload.nome,
        cognome: payload.cognome,
        ruolo: payload.ruolo,
    };*/

    //let emp = table(format!("{}.employees", schema_name))

    diesel::sql_query(query)
        .bind::<Text, _>(payload.nome.clone())
        .bind::<Text, _>(payload.cognome.clone())
        .bind::<Text, _>(payload.ruolo.clone())
        .execute(&mut conn)
        .map_err(|e| format!("Failed to insert dipendente: {}", e))?;

    Ok(Json("Employee added successfully"))
}


async fn update_employee(
    session: WritableSession,
    State(pool): State<Arc<DbPool>>,
    Json(payload): Json<UpdateEmployeeRequest>,
) -> Result<impl IntoResponse, String> {
    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let mut conn = pool.get().map_err(|e| {
        format!("Failed to get DB connection: {}", e)
    })?;

    use subschemas::employees::dsl::*;

    print!("id:{}", payload.id);

    /*let target = employees.filter(id.eq(payload.id));

    let mut update_request = UpdateEmployeeFields {
        nome: None,
        cognome: None,
        ruolo: None,
    };


    if let Some(new_nome) = payload.nome {
        update_request.nome = Some(new_nome);
    }

    if let Some(new_cognome) = payload.cognome {
        update_request.cognome = Some(new_cognome);
    }

    if let Some(new_ruolo) = payload.ruolo {
        update_request.ruolo = Some(new_ruolo);
    }

    println!("{:?}", update_request);

    
    diesel::update(target)
        .set(update_request)
        .execute(&mut conn).map_err(|e| {
            format!("Failed to update dipendente: {}", e)
        })?;*/

    let schema_name = format!("user_{}", user_id);
    let table_name = format!("{}.employees", schema_name);

    let query = format!(
        r#"UPDATE "{}"."employees" SET
            nome = COALESCE($1, nome),
            cognome = COALESCE($2, cognome),
            ruolo = COALESCE($3, ruolo)
            WHERE id = $4"#,
        schema_name
    );

    // Esecuzione della query con parametri
    diesel::sql_query(query)
        .bind::<Nullable<Text>, _>(payload.nome.clone())
        .bind::<Nullable<Text>, _>(payload.cognome.clone())
        .bind::<Nullable<Text>, _>(payload.ruolo.clone())
        .bind::<diesel::sql_types::Int4, _>(payload.id)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update dipendente: {}", e))?;

    Ok(Json("Employee updated successfully"))
}

async fn get_employee(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetEmployeeDataQuery>,
) -> Result<Json<Vec<models::Dipendenti>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    println!("{:?}", &session.get::<String>("user_id"));

    /*let schema_name = format!("user_{}", user_id);
    utils::set_search_path(&mut conn, &schema_name).map_err(|e| format!("Failed to set search path: {}", e))?;

    use subschemas::employees::dsl::*;

    let mut query_builder = employees.into_boxed();

    query_builder = query_builder.filter(id.eq(params.id));
    

    let results = query_builder
    .load::<models::Employee>(&mut conn)
    .map_err(|e| {
        format!("Failed to load employee: {}", e)
    })?;*/

    let schema_name = format!("user_{}", user_id);

    let query = format!(
        r#"SELECT id, nome, cognome, ruolo FROM "{}"."employees" WHERE id = $1"#,
        schema_name
    );

    let results = sql_query(query)
        .bind::<diesel::sql_types::Int4, _>(params.id)
        .load::<models::Dipendenti>(&mut conn)
        .map_err(|e| format!("Failed to execute query: {}", e))?;


    Ok(Json(results))
}

async fn remove_employee(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveEmployeeQuery>,
) -> Result<impl IntoResponse, String> {
    let user_id = match session.get::<String>("user_id") {
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string()),
    };

    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let schema_name = format!("user_{}", user_id);
    let query = format!(r#"DELETE FROM "{}"."employees" WHERE id = $1"#, schema_name);

    diesel::sql_query(query)
        .bind::<diesel::sql_types::Integer, _>(params.id)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to delete employee: {}", e))?;

    Ok((StatusCode::OK, Json("Employee removed successfully")))
}

pub async fn session_check(
    session: ReadableSession,
) -> impl IntoResponse {
    match <axum_sessions::async_session::Session as Clone>::clone(&session).validate() {
        Some(_) => Json("".to_string()),
        None => Json("Unauthorized".to_string())
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let redis_client = Client::open("redis://localhost:6379").unwrap();
    let redis_store = RedisSessionStore::from_client(redis_client).with_prefix("session");


    let session_layer = SessionLayer::new(redis_store, "ti6)=BrVDAmX+@x2chw5,}gm1:5nj~V9_rRrj}%4T)=]>PzAn3#,a.b6MqE?-LQG".as_bytes())
        .with_cookie_name("session")
        .with_secure(true)
        .with_http_only(true)
        .with_persistence_policy(axum_sessions::PersistencePolicy::ChangedOnly)
        .with_same_site_policy(axum_sessions::SameSite::None);

    let app = Router::new()
        .route("/check_session", get(session_check))
        .route("/register", post(register))
        .route("/get_user_data", get(get_user_data))
        .route("/login", post(login))
        .route("/add_employee", post(add_employee))
        .route("/update_employee", post(update_employee))
        .route("/get_employee", get(get_employee))
        .route("/remove_employee", post(remove_employee))
        .layer(session_layer)
        .with_state(pool.into());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}