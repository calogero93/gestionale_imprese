use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection, RunQueryDsl};
use crate::{models, request_states::*, schema, utils::*};


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub async fn add_user(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddUserRequest>,
) -> anyhow::Result<impl IntoResponse, String> {

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::utentis;

    let new_user = models::NewUtente {
        username: payload.username.clone(),
        password: hashing(&payload.password.clone()).to_string(),
        nome: payload.name.clone(),
        cognome: payload.surname.clone(),
        impresa_id: payload.company_id,
        utente: payload.username.clone(),
        autorizazzione: Some(payload.state),
        primo_login: None,
        super_utente: None,
    };

    let user_id: i32 = diesel::insert_into(utentis::table)
        .values(&new_user)
        .returning(utentis::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert user: {}", e))?;

    


    Ok(Json(format!("Utente {} registrato con successo", payload.username)).into_response())
    
}

pub async fn add_autovetture(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddAutovettureRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::autovettures;

    let new_autovetture = models::NewAutovettura {
        descrizione: payload.descrizione.clone(),
        modello: payload.modello.clone(),
        targa: payload.targa.clone(),
        tipo_proprieta: payload.tipo_proprieta,
        proprieta: payload.proprieta.clone(),
        impresa_id: payload.impresa_id,
        data_dimissioni: payload.data_dimissioni,
        rfid1: payload.rfid1.clone(),
        rfid2: payload.rfid2.clone(),
    };

    let autovettura_id: i32 = diesel::insert_into(autovettures::table)
        .values(&new_autovetture)
        .returning(autovettures::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert autovettura: {}", e))?;

    Ok(Json(format!("Autovettura registrata con successo")).into_response())
}

pub async fn add_dipendenti(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddDipendentiRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let super_user = match session.get::<bool>("super_utente"){
        Some(super_user) => super_user,
        None => false
    };

    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let user_id = user_id.parse().unwrap();
    let utente = get_utente(&mut conn, user_id)?;

    let imprese_associate = get_imprese_associate_utenti(&mut conn, user_id)?;

    let imprese_associate_id = imprese_associate.into_iter().map(|impresa| impresa.impresa_id).collect::<Vec<i32>>();

    if !(payload.impresa_id == utente.impresa_id || imprese_associate_id.contains(&payload.impresa_id) || super_user) {
        return Err("Unauthorized".to_string())
    }


    use schema::dipendentis;

    let new_dipendente = models::NewDipendente {
        nome: payload.nome.clone(),
        cognome: payload.cognome.clone(),
        matricola: payload.matricola.clone(),
        data_di_nascita: payload.data_di_nascita.clone(),
        luogo_di_nascita: payload.luogo_di_nascita.clone(),
        codice_fiscale: payload.codice_fiscale.clone(),
        impresa_id: payload.impresa_id,
        qualifica: payload.qualifica,
        mansione: payload.mansione,
        data_dimissioni: payload.data_dimissioni,
        rfid1: payload.rfid1.clone(),
        rfid2: payload.rfid2.clone(),
    };

    let dipendente_id: i32 = diesel::insert_into(dipendentis::table)
        .values(&new_dipendente)
        .returning(dipendentis::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert dipendente: {}", e))?;

    Ok(Json(format!("Dipendente registrato con successo")).into_response())
}

pub async fn add_imprese_associate_utentis(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddImpreseAssociateUtentisRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };


    use schema::imprese_associate_utentis;

    let new_associate = models::NewImpreseAssociateUtente {
        utente_id: payload.utente_id,
        impresa_id: payload.impresa_id,
    };

    let associate_id: i32 = diesel::insert_into(imprese_associate_utentis::table)
        .values(&new_associate)
        .returning(imprese_associate_utentis::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert associate: {}", e))?;

    Ok(Json(format!("Associazione registrata con successo")).into_response())
}

pub async fn add_imprese_collegate(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddImpreseCollegateRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::imprese_collegates;

    let new_collegato = models::NewImpreseCollegata {
        impresa_id: payload.impresa_id,
        ruolo_impresa: payload.ruolo_impresa.clone(),
    };

    let collegato_id: i32 = diesel::insert_into(imprese_collegates::table)
        .values(&new_collegato)
        .returning(imprese_collegates::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert collegato: {}", e))?;

    Ok(Json(format!("Collegamento registrato con successo")).into_response())
}

pub async fn add_imprese(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddImpreseRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::impreses;

    let new_impresa = models::NewImpresa {
        ragione_sociale: payload.ragione_sociale.clone(),
        indirizzo: payload.indirizzo.clone(),
        partita_iva: payload.partita_iva.clone()
    };

    let _: i32 = diesel::insert_into(impreses::table)
        .values(&new_impresa)
        .returning(impreses::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert impresa: {}", e))?;

    Ok(Json(format!("Impresa registrata con successo")).into_response())
}

pub async fn add_mansioni(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddMansioniRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::mansionis;

    let new_mansione = models::NewMansione {
        descrizione: payload.descrizione.clone(),
    };

    let mansione_id: i32 = diesel::insert_into(mansionis::table)
        .values(&new_mansione)
        .returning(mansionis::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert mansione: {}", e))?;

    Ok(Json(format!("Mansione registrata con successo")).into_response())
}

pub async fn add_mezzi(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddMezziRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::mezzis;

    let new_mezzo = models::NewMezzo {
        descrizione: payload.descrizione.clone(),
        modello: payload.modello.clone(),
        tipo_proprieta: payload.tipo_proprieta,
        proprieta: payload.proprieta.clone(),
        impresa_id: payload.impresa_id,
        data_dimissioni: payload.data_dimissioni,
        rfid1: payload.rfid1.clone(),
        rfid2: payload.rfid2.clone(),
    };

    let mezzo_id: i32 = diesel::insert_into(mezzis::table)
        .values(&new_mezzo)
        .returning(mezzis::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert mezzo: {}", e))?;

    Ok(Json(format!("Mezzo registrato con successo")).into_response())
}

pub async fn add_opere(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddOpereRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::operes;

    let new_opera = models::NewOpera {
        descrizione: payload.descrizione.clone(),
    };

    let opera_id: i32 = diesel::insert_into(operes::table)
        .values(&new_opera)
        .returning(operes::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert opera: {}", e))?;

    Ok(Json(format!("Opera registrata con successo")).into_response())
}

pub async fn add_qualifiche(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddQualificheRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::qualifiches;

    let new_qualifica = models::NewQualifica {
        descrizione: payload.descrizione.clone(),
    };

    let qualifica_id: i32 = diesel::insert_into(qualifiches::table)
        .values(&new_qualifica)
        .returning(qualifiches::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert qualifica: {}", e))?;

    Ok(Json(format!("Qualifica registrata con successo")).into_response())
}

pub async fn add_tipi_proprieta(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<AddTipiProprietaRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    let _ = match session.get::<bool>("super_utente"){
        Some(super_user) => {
            if !super_user {
                return Err("Unauthorized".to_string())
            }
        },
        None => return Err("Unauthorized".to_string())
    };

    use schema::tipi_proprietas;

    let new_tipo_proprieta = models::NewTipoProprieta {
        descrizione: payload.descrizione.clone(),
    };

    let tipo_proprieta_id: i32 = diesel::insert_into(tipi_proprietas::table)
        .values(&new_tipo_proprieta)
        .returning(tipi_proprietas::id)
        .get_result(&mut conn)
        .map_err(|e| format!("Failed to insert tipo_proprieta: {}", e))?;

    Ok(Json(format!("Tipo Proprieta registrato con successo")).into_response())
}