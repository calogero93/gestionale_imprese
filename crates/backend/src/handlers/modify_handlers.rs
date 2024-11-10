use std::sync::Arc;

use api_error::APIError;
use api_utils::{get_autovettura, get_imprese_associate_utenti, get_mezzo, get_utente};
use axum::{extract::State, response::IntoResponse, Json};
use chrono::{DateTime, FixedOffset, NaiveTime};
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use hyper::StatusCode;
use prisma::PrismaClient;
use tokio::sync::Mutex;
use crate::{models, request_states::*, schema, utils::*};


pub async fn update_settimanale(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateSettimanale>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(data_settimanale) = payload.data_settimanale.clone() {
        update_params.push(prisma::settimanale::SetParam::SetDataSettimanale(data_settimanale));
    }
    if let Some(utente_id) = payload.utente_id {
        update_params.push(prisma::settimanale::SetParam::SetUtenteId(utente_id));
    }
    if let Some(luogo_di_nascita) = payload.luogo_di_nascita.clone() {
        update_params.push(prisma::settimanale::SetParam::SetLuogoDiNascita(luogo_di_nascita));
    }
    if let Some(data_di_nascita) = payload.data_di_nascita.clone() {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_di_nascita = DateTime::from_naive_utc_and_offset(data_di_nascita.and_time(t), FixedOffset::east(0));
        update_params.push(prisma::settimanale::SetParam::SetDataDiNascita(data_di_nascita));
    }
    if let Some(tipo_proprieta) = payload.tipo_proprieta {
        update_params.push(prisma::settimanale::SetParam::SetTipoProprieta(tipo_proprieta));
    }
    if let Some(proprieta) = payload.proprieta.clone() {
        update_params.push(prisma::settimanale::SetParam::SetProprieta(proprieta));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::settimanale::SetParam::SetImpresaId(impresa_id));
    }
    if let Some(opera_id) = payload.opera_id {
        update_params.push(prisma::settimanale::SetParam::SetOperaId(opera_id));
    }
    if let Some(mezzo_id) = payload.mezzo_id {
        update_params.push(prisma::settimanale::SetParam::SetMezzoId(Some(mezzo_id)));
    }
    if let Some(autovettura_id) = payload.autovettura_id {
        update_params.push(prisma::settimanale::SetParam::SetAutovetturaId(autovettura_id));
    }
    if let Some(matricola) = payload.matricola.clone() {
        update_params.push(prisma::settimanale::SetParam::SetMatricola(Some(matricola)));
    }
    if let Some(targa) = payload.targa.clone() {
        update_params.push(prisma::settimanale::SetParam::SetTarga(targa));
    }

    client.settimanale()
        .update(
            prisma::settimanale::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update settimanale: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Settimanale aggiornato con successo"))
}


pub async fn update_qualifiche(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateQualificheRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::qualifica::SetParam::SetDescrizione(descrizione));
    }

    client.qualifica()
        .update(
            prisma::qualifica::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update qualifica: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Qualifica aggiornata con successo"))
}


pub async fn update_mansioni(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateMansioniRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::mansione::SetParam::SetDescrizione(descrizione));
    }

    client.mansione()
        .update(
            prisma::mansione::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update mansione: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Mansione aggiornata con successo"))
}


pub async fn update_opere(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateOpereRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::opere::SetParam::SetDescrizione(descrizione));
    }

    client.opere()
        .update(
            prisma::opere::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update opera: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Opera aggiornata con successo"))
}


pub async fn update_tipi_proprieta(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateTipiProprietaRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::tipi_proprieta::SetParam::SetDescrizione(descrizione));
    }

    client.tipi_proprieta()
        .update(
            prisma::tipi_proprieta::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update tipo_proprieta: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Tipo Proprieta aggiornato con successo"))
}


pub async fn update_imprese(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateImpreseRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(ragione_sociale) = payload.ragione_sociale.clone() {
        update_params.push(prisma::imprese::SetParam::SetRagioneSociale(ragione_sociale));
    }
    if let Some(indirizzo) = payload.indirizzo.clone() {
        update_params.push(prisma::imprese::SetParam::SetIndirizzo(indirizzo));
    }
    if let Some(partita_iva) = payload.partita_iva.clone() {
        update_params.push(prisma::imprese::SetParam::SetPartitaIva(partita_iva));
    }

    client.imprese()
        .update(
            prisma::imprese::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update impresa: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Impresa aggiornata con successo"))
}

pub async fn update_utenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateUtentiRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(username) = payload.username.clone() {
        update_params.push(prisma::utenti::SetParam::SetUsername(username));
    }
    if let Some(password) = payload.password.clone() {
        update_params.push(prisma::utenti::SetParam::SetPassword(password));
    }
    if let Some(nome) = payload.nome.clone() {
        update_params.push(prisma::utenti::SetParam::SetNome(Some(nome)));
    }
    if let Some(cognome) = payload.cognome.clone() {
        update_params.push(prisma::utenti::SetParam::SetCognome(Some(cognome)));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::utenti::SetParam::SetImpresaId(Some(impresa_id)));
    }
    if let Some(utente) = payload.utente.clone() {
        update_params.push(prisma::utenti::SetParam::SetUtente(Some(utente)));
    }
    if let Some(autorizzazione) = payload.autorizazzione {
        update_params.push(prisma::utenti::SetParam::SetAutorizzazione(Some(autorizzazione)));
    }
    if let Some(primo_login) = payload.primo_login {
        update_params.push(prisma::utenti::SetParam::SetPrimoLogin(Some(primo_login)));
    }
    if let Some(super_utente) = payload.super_utente {
        update_params.push(prisma::utenti::SetParam::SetSuperUtente(Some(super_utente)));
    }

    client.utenti()
        .update(
            prisma::utenti::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update utente: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Utente aggiornato con successo"))
}

pub async fn update_dipendenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateDipendentiRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(nome) = payload.nome.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetNome(nome));
    }
    if let Some(cognome) = payload.cognome.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetCognome(cognome));
    }
    if let Some(matricola) = payload.matricola.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetMatricola(Some(matricola)));
    }
    if let Some(data_di_nascita) = payload.data_di_nascita {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_di_nascita = DateTime::from_naive_utc_and_offset(data_di_nascita.and_time(t), FixedOffset::east(0));
        update_params.push(prisma::dipendenti::SetParam::SetDataDiNascita(data_di_nascita));
    }
    if let Some(luogo_di_nascita) = payload.luogo_di_nascita.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetLuogoDiNascita(luogo_di_nascita));
    }
    if let Some(codice_fiscale) = payload.codice_fiscale.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetCodiceFiscale(codice_fiscale));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::dipendenti::SetParam::SetImpresaId(impresa_id));
    }
    if let Some(qualifica) = payload.qualifica {
        update_params.push(prisma::dipendenti::SetParam::SetQualificaId(qualifica));
    }
    if let Some(mansione) = payload.mansione {
        update_params.push(prisma::dipendenti::SetParam::SetMansioneId(mansione));
    }
    if let Some(data_dimissioni) = payload.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni = DateTime::from_naive_utc_and_offset(data_dimissioni.and_time(t), FixedOffset::east(0));
        update_params.push(prisma::dipendenti::SetParam::SetDataDimissioni(Some(data_dimissioni)));
    }
    if let Some(rfid1) = payload.rfid1.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetRfid1(Some(rfid1)));
    }
    if let Some(rfid2) = payload.rfid2.clone() {
        update_params.push(prisma::dipendenti::SetParam::SetRfid2(Some(rfid2)));
    }

    client.dipendenti()
        .update(
            prisma::dipendenti::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update dipendente: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Dipendente aggiornato con successo"))
}

pub async fn update_imprese_associate_utentis(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateImpreseAssociateUtentisRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(utente_id) = payload.utente_id {
        update_params.push(prisma::imprese_associate_utenti::SetParam::SetUtenteId(utente_id));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::imprese_associate_utenti::SetParam::SetImpresaId(impresa_id));
    }

    client.imprese_associate_utenti()
        .update(
            prisma::imprese_associate_utenti::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update impresa associata utente: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Impresa associata all'utente aggiornata con successo"))
}

pub async fn update_mezzi(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateMezziRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::mezzi::SetParam::SetDescrizione(Some(descrizione)));
    }
    if let Some(modello) = payload.modello.clone() {
        update_params.push(prisma::mezzi::SetParam::SetModello(modello));
    }
    if let Some(tipo_proprieta) = payload.tipo_proprieta {
        update_params.push(prisma::mezzi::SetParam::SetTipoProprieta(tipo_proprieta));
    }
    if let Some(proprieta) = payload.proprieta.clone() {
        update_params.push(prisma::mezzi::SetParam::SetProprieta(proprieta));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::mezzi::SetParam::SetImpresaId(impresa_id));
    }
    if let Some(data_dimissioni) = payload.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni = DateTime::from_naive_utc_and_offset(data_dimissioni.and_time(t), FixedOffset::east(0));
        update_params.push(prisma::mezzi::SetParam::SetDataDimissioni(data_dimissioni));
    }
    if let Some(rfid1) = payload.rfid1.clone() {
        update_params.push(prisma::mezzi::SetParam::SetRfid1(rfid1));
    }
    if let Some(rfid2) = payload.rfid2.clone() {
        update_params.push(prisma::mezzi::SetParam::SetRfid2(rfid2));
    }

    client.mezzi()
        .update(
            prisma::mezzi::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update mezzo: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Mezzo aggiornato con successo"))
}


pub async fn update_autovetture(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateAutovettureRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(descrizione) = payload.descrizione.clone() {
        update_params.push(prisma::autovetture::SetParam::SetDescrizione(Some(descrizione)));
    }
    if let Some(modello) = payload.modello.clone() {
        update_params.push(prisma::autovetture::SetParam::SetModello(modello));
    }
    if let Some(targa) = payload.targa.clone() {
        update_params.push(prisma::autovetture::SetParam::SetTarga(targa));
    }
    if let Some(tipo_proprieta) = payload.tipo_proprieta {
        update_params.push(prisma::autovetture::SetParam::SetTipoProprieta(tipo_proprieta));
    }
    if let Some(proprieta) = payload.proprieta.clone() {
        update_params.push(prisma::autovetture::SetParam::SetProprieta(proprieta));
    }
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::autovetture::SetParam::SetImpresaId(impresa_id));
    }
    if let Some(data_dimissioni) = payload.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni = DateTime::from_naive_utc_and_offset(data_dimissioni.and_time(t), FixedOffset::east(0));
        update_params.push(prisma::autovetture::SetParam::SetDataDimissioni(data_dimissioni));
    }
    if let Some(rfid1) = payload.rfid1.clone() {
        update_params.push(prisma::autovetture::SetParam::SetRfid1(rfid1));
    }
    if let Some(rfid2) = payload.rfid2.clone() {
        update_params.push(prisma::autovetture::SetParam::SetRfid2(rfid2));
    }

    client.autovetture()
        .update(
            prisma::autovetture::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update autovettura: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Autovettura aggiornata con successo"))
}


pub async fn update_imprese_collegate(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Json(payload): Json<UpdateImpreseCollegateRequest>,
) -> Result<impl IntoResponse, APIError> {
    let client = prisma.lock().await;

    let mut update_params = vec![];
    if let Some(impresa_id) = payload.impresa_id {
        update_params.push(prisma::imprese_collegate::SetParam::SetImpresaId(impresa_id));
    }
    if let Some(ruolo_impresa) = payload.ruolo_impresa.clone() {
        update_params.push(prisma::imprese_collegate::SetParam::SetRuoloImpresa(ruolo_impresa));
    }

    client.imprese_collegate()
        .update(
            prisma::imprese_collegate::UniqueWhereParam::IdEquals(payload.id),
            update_params,
        )
        .exec()
        .await
        .map_err(|err| APIError {
            message: format!("Failed to update impresa collegata: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json("Impresa collegata aggiornata con successo"))
}
