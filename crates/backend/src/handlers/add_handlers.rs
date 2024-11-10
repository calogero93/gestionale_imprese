use std::{str::FromStr, sync::Arc};

use api_error::APIError;
use api_utils::{get_imprese_associate_utenti, get_utente, hashing};
use axum::{extract::State, response::IntoResponse, Json};
use axum_sessions::{async_session::chrono::NaiveDate, extractors::ReadableSession};
use chrono::{DateTime, FixedOffset, NaiveTime};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection, RunQueryDsl};
use hyper::StatusCode;
use prisma::PrismaClient;
use tokio::sync::Mutex;
use crate::{models::{self, NewSettimanale}, request_states::*, schema, utils::*};



pub async fn add_settimanale(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddSettimanaleRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let data_di_nascita = DateTime::from_naive_utc_and_offset(payload.data_di_nascita.and_time(t), FixedOffset::east(0));

    client.settimanale().create(
        payload.data_settimanale.clone(),
        payload.luogo_di_nascita.clone(),
        data_di_nascita,
        payload.proprieta.clone(),
        payload.targa,
        prisma::utenti::UniqueWhereParam::IdEquals(payload.utente_id),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::opere::UniqueWhereParam::IdEquals(payload.opera_id),
        prisma::autovetture::UniqueWhereParam::IdEquals(payload.autovettura_id),
        prisma::tipi_proprieta::UniqueWhereParam::IdEquals(payload.tipo_proprieta),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert settimanale: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Settimanale aggiunto con successo"))
}




pub async fn add_user(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddUserRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.utenti().create(
        payload.username.clone(),
        hashing(&payload.password.clone()).to_string(),
        //payload.name.clone(),
        //payload.surname.clone(),
        //prisma::imprese::UniqueWhereParam::IdEquals(payload.company_id),
        //Some(payload.state),
        //None,
        //None,
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert user: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json(format!("Utente {} registrato con successo", payload.username)))
}


pub async fn add_autovetture(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddAutovettureRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.autovetture().create(
        payload.modello.clone(),
        payload.targa.clone(),
        payload.proprieta,
        DateTime::parse_from_rfc3339(&payload.data_dimissioni).unwrap(),
        payload.rfid1.clone(),
        payload.rfid2.clone(),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::tipi_proprieta::UniqueWhereParam::IdEquals(payload.tipo_proprieta),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert autovettura: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Autovettura registrata con successo"))
}


pub async fn add_dipendenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddDipendentiRequest>,
) -> Result<impl IntoResponse, APIError> {

    let client = prisma.lock().await;
    let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let data_di_nascita = DateTime::from_naive_utc_and_offset(payload.data_di_nascita.and_time(t), FixedOffset::east(0));

    client.dipendenti().create(
        payload.nome.clone(),
        payload.cognome.clone(),
        data_di_nascita,
        payload.luogo_di_nascita.clone(),
        payload.codice_fiscale.clone(),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::qualifica::UniqueWhereParam::IdEquals(payload.qualifica),
        prisma::mansione::UniqueWhereParam::IdEquals(payload.mansione),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert dipendente: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Dipendente registrato con successo"))
}


pub async fn add_imprese_associate_utentis(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddImpreseAssociateUtentisRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.imprese_associate_utenti().create(
        prisma::utenti::UniqueWhereParam::IdEquals(payload.utente_id),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to associate impresa to utente: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Associazione registrata con successo"))
}


pub async fn add_imprese_collegate(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddImpreseCollegateRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.imprese_collegate().create(
        payload.ruolo_impresa.clone(),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.imprese_collegata_id),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to associate imprese: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Collegamento registrato con successo"))
}


pub async fn add_imprese(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddImpreseRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.imprese().create(
        payload.ragione_sociale.clone().unwrap(),
        payload.indirizzo.clone(),
        payload.partita_iva.clone(),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert impresa: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Impresa registrata con successo"))
}


pub async fn add_mansioni(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddMansioniRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.mansione().create(
        payload.descrizione.clone(),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert mansione: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Mansione registrata con successo"))
}


pub async fn add_mezzi(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddMezziRequest>,
) -> Result<impl IntoResponse, APIError> {

    let client = prisma.lock().await;

    let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let data_dimissioni = DateTime::from_naive_utc_and_offset(payload.data_dimissioni.and_time(t), FixedOffset::east(0));

    client.mezzi().create(
        payload.modello.clone(),
        payload.proprieta.clone(),
        data_dimissioni,
        payload.rfid1.clone(),
        payload.rfid2.clone(),
        prisma::imprese::UniqueWhereParam::IdEquals(payload.impresa_id),
        prisma::tipi_proprieta::UniqueWhereParam::IdEquals(payload.tipo_proprieta),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert mezzo: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Mezzo registrato con successo"))
}


pub async fn add_opere(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddOpereRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.opere().create(
        payload.descrizione.clone(),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert opera: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Opera registrata con successo"))
}


pub async fn add_qualifiche(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddQualificheRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.qualifica().create(
        payload.descrizione.clone(),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert qualifica: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Qualifica registrata con successo"))
}


pub async fn add_tipi_proprieta(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<AddTipiProprietaRequest>,
) -> Result<impl IntoResponse, APIError> {

    if session.get::<bool>("super_utente").unwrap_or(false) == false {
        return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        });
    }

    let client = prisma.lock().await;

    client.tipi_proprieta().create(
        payload.descrizione.clone(),
        vec![]
    ).exec()
    .await
    .map_err(|err| APIError {
        message: format!("Failed to insert tipo_proprieta: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(500),
    })?;

    Ok(Json("Tipo Proprieta registrato con successo"))
}
