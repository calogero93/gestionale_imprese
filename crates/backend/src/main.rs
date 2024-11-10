use async_redis_session::RedisSessionStore;
use auth::{auth_layer::guard, jwt::encode_jwt};
use axum::{
    extract::{Json, Query, State}, handler::Handler, middleware, response::{IntoResponse, Response}, routing::{get, post}, Router
};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Offset};
use entities::DipendentiEntity;
use gestionale_imprese::utils::api_error::APIError;
use handlers::{add_handlers::*, get_handlers::*, remove_handlers::*, modify_handlers::*};
use hyper::StatusCode;
use redis::Client;
use request_states::{AddEmployeeRequest, ChangePasswordRequest, GetEmployeeDataQuery, LoginRequest, RegisterRequest, RemoveEmployeeQuery};
use response_states::LoginResponse;
use std::{net::SocketAddr, sync::Arc};
use dotenvy::dotenv;
use axum_sessions::{extractors::{ReadableSession, WritableSession}, SessionLayer};
use tokio::sync::Mutex;
use std::env;

mod schema;
mod models;
mod utils;
mod request_states;
mod response_states;
pub mod handlers;
mod query_states;
mod remove_state;
mod auth;
pub mod entities;

use once_cell::sync::OnceCell;
use prisma::{instance_prisma_client, utenti::WhereParam, PrismaClient};


// Funzione di esempio per registrare un utente
async fn register(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    payload: Json<RegisterRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;
    
    let new_user = client.utenti().create(
        payload.username.clone(),
        payload.password.clone(),
        vec![]
    ).exec().await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::BAD_REQUEST,
        error_code: Some(401),
    })?;

    Ok(Json(format!("User {} registered", payload.username)).into_response())
}


async fn login(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    payload: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    let client = prisma.lock().await;

    let user = client
        .utenti()
        .find_first(vec![
            WhereParam::Username(prisma::read_filters::StringFilter::Equals(payload.username.clone())),
            WhereParam::Password(prisma::read_filters::StringFilter::Equals(payload.password.clone())),
        ])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    if user.is_none() {
        return Err(APIError {
            message: "Dati Utente non trovati".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        });
    }

    // Corregge il tipo di errore ritornato da `encode_jwt` usando `map_err` per mappare su `APIError`
    let token = encode_jwt(user.clone().unwrap().username).map_err(|_| APIError {
        message: "Failed to login".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(403),
    })?;

    Ok(Json(LoginResponse {
        message: "Login avvenuto con successo".to_string(),
        token,
        first_login: user.clone().unwrap().primo_login,
        auth: user.unwrap().super_utente,
    }))
}


async fn add_employee(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddEmployeeRequest>,
) -> Result<impl IntoResponse, APIError> {

    let client = prisma.lock().await;

    let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

    client.dipendenti().create(
        payload.nome.clone(),
        payload.cognome.clone(),
        DateTime::from_naive_utc_and_offset(payload.data_di_nascita.clone().and_time(t), FixedOffset::east(0)),
        payload.luogo_di_nascita.clone(),
        payload.codice_fiscale.clone(),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::qualifica::UniqueWhereParam::IdEquals(payload.qualifica_id),
        prisma::mansione::UniqueWhereParam::IdEquals(payload.mansione_id),
        vec![]

    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert dipendente: {}", err),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(500),
    })?;
    

    Ok(Json("Employee added successfully"))
}


async fn get_employee(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetEmployeeDataQuery>,
) -> Result<Json<Vec<DipendentiEntity>>, APIError> {
    let client = prisma.lock().await;

    let employee = client.dipendenti().find_first(vec![
        prisma::dipendenti::WhereParam::Id(prisma::read_filters::IntFilter::Equals(params.id))
    ]).exec()
    .await
    .map_err(|err|APIError {
        message: err.to_string(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    })?;

    if employee.is_none() {
        return Err(APIError {
            message: "Dati Dipendente non trovati".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        });
    }

    Ok(Json(vec![employee.unwrap()]))
}

async fn remove_employee(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveEmployeeQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client
        .dipendenti()
        .delete(
            prisma::dipendenti::UniqueWhereParam::IdEquals(params.id)
        )
        .exec()
        .await
        .map_err(|e| format!("Failed to delete employee: {}", e))?;

    Ok((StatusCode::OK, Json("Employee removed successfully")))
}

async fn change_password(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<ChangePasswordRequest>
) -> Result<Json<String>, APIError> {
    let client = prisma.lock().await;

    let user = client.utenti().find_first(vec![
        prisma::utenti::WhereParam::Id(prisma::read_filters::IntFilter::Equals(payload.id))
    ]).exec()
    .await
    .map_err(|err| 
        APIError {
            message: format!("Utente non trovato: {}", err),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(500),
        })?;

    if user.is_none() {
        return Err(APIError {
            message: "Dati Utente non trovati".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        });
    }
    let user = user.unwrap();
    if user.password == payload.old_password {
        client.utenti().update(
            prisma::utenti::UniqueWhereParam::IdEquals(user.id),
            vec![prisma::utenti::SetParam::SetPassword(payload.new_password.clone())]
        )
        .exec()
        .await
        .map_err(|err| 
            APIError {
                message: format!("Errore nell'aggiornamento della password: {}", err),
                status_code: StatusCode::NOT_FOUND,
                error_code: Some(500),
            }           
        )?;

        Ok(Json("Password Cambiata con successo".to_string()))
    } else {
        Err(APIError {
            message: "La vecchia password non è corretta".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(500),
        })

    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let prisma = get_prisma_client().await?;

    let app = Router::new()
        .route("/register", post(register))
        //.route("/get_user_data", get(get_user_data))
        .route("/login", post(login))
        //.route_layer(middleware::from_fn(guard))
        .route("/add_employee", post(add_employee).layer(middleware::from_fn(guard)))
        .route("/get_employee", get(get_employee))
        .route("/remove_employee", post(remove_employee))
        .route("/change_password", post(change_password))
        .route("/add_utente", post(add_user))
        .route("/update_utente", post(update_utenti))
        .route("/get_utenti", get(get_utenti))
        .route("/remove_utente", post(remove_utente))
        .route("/add_dipendente", post(add_dipendenti))
        .route("/update_dipendente", post(update_dipendenti))
        .route("/get_dipendenti", get(get_dipendente))
        .route("/remove_dipendente", post(remove_dipendente))
        .route("/add_mezzo", post(add_mezzi))
        .route("/update_mezzo", post(update_mezzi))
        .route("/get_mezzi", get(get_mezzi))
        .route("/remove_mezzo", post(remove_mezzo))
        .route("/add_autovettura", post(add_autovetture))
        .route("/update_autovettura", post(update_autovetture))
        .route("/get_autovetture", get(get_autovetture))
        .route("/remove_autovettura", post(remove_autovettura))
        .route("/add_impresa", post(add_imprese))
        .route("/update_impresa", post(update_imprese))
        .route("/get_imprese", get(get_imprese))
        .route("/remove_impresa", post(remove_impresa))
        .route("/add_qualifica", post(add_qualifiche))
        .route("/update_qualifica", post(update_qualifiche))
        .route("/get_qualifiche", get(get_qualifiche))
        .route("/get_qualifica", get(get_qualifica))
        .route("/remove_qualifica", post(remove_qualifica))
        .route("/add_mansione", post(add_mansioni))
        .route("/update_mansione", post(update_mansioni))
        .route("/get_mansioni", get(get_mansioni))
        .route("/remove_mansione", post(remove_mansione))
        .route("/add_opera", post(add_opere))
        .route("/update_opera", post(update_opere))
        .route("/get_opere", get(get_opere))
        .route("/remove_opera", post(remove_opera))
        .route("/add_tipo_proprieta", post(add_tipi_proprieta))
        .route("/update_tipo_proprieta", post(update_tipi_proprieta))
        .route("/get_tipi_proprieta", get(get_tipi_proprieta))
        .route("/remove_tipo_proprieta", post(remove_tipo_proprieta))
        .route("/add_imprese_associate_utenti", post(add_imprese_associate_utentis))
        .route("/update_imprese_associate_utenti", post(update_imprese_associate_utentis))
        .route("/get_imprese_associate_utenti", get(handlers::get_handlers::get_imprese_associate_utenti))
        .route("/remove_imprese_associate_utenti", post(remove_imprese_associate_utenti))
        .route("/add_imprese_collegate", post(add_imprese_collegate))
        .route("/update_imprese_collegate", post(update_imprese_collegate))
        .route("/get_imprese_collegate", get(handlers::get_handlers::get_imprese_collegate))
        .route("/remove_imprese_collegate", post(remove_imprese_collegate))
        .route("/get_utente", get(handlers::get_handlers::get_utente))
        .route("/get_dipendente", get(get_dipendente))
        .route("/get_mezzo", get(get_mezzo))
        .route("/get_autovettura", get(get_autovettura))
        .route("/get_impresa", get(get_impresa))
        .route("/get_mansione", get(get_mansione))
        .route("/get_opera", get(get_opera))
        .route("/get_tipo_proprieta", get(get_tipo_proprieta))
        .route("/get_impresa_associata_utente", get(handlers::get_handlers::get_impresa_associata_utente))
        .route("/get_impresa_collegata", get(handlers::get_handlers::get_impresa_collegata))
        .route("/add_settimanale", post(add_settimanale))
        .route("/get_settimanale", get(get_settimanale))
        .route("/update_settimanale", post(update_settimanale))
        .route("/remove_settimanale", post(remove_settimanale))
        .with_state(Arc::new(Mutex::new(prisma.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");

    Ok(())
}


pub async fn get_prisma_client() -> anyhow::Result<&'static PrismaClient> {
    static PRISMA_CLIENT: OnceCell<PrismaClient> = OnceCell::new();

    match PRISMA_CLIENT.get() {
        Some(client) => Ok(client),
        None => {
            let client = instance_prisma_client(&env::var("DATABASE_URL")?)
                .await
                .map_err(|err| anyhow::anyhow!(format!("{err:?}")))?;
            PRISMA_CLIENT
                .set(client)
                .map_err(|_| anyhow::anyhow!("Expected to set prisma client"))?;
            PRISMA_CLIENT
                .get()
                .ok_or(anyhow::anyhow!("Expected to get prisma client at this point"))
        }
    }
}
