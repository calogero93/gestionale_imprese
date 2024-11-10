use std::sync::Arc;

use api_error::APIError;
use axum::{extract::{Query, State}, Json};
use axum_sessions::extractors::ReadableSession;
use chrono::{DateTime, FixedOffset, NaiveTime};
use hyper::StatusCode;
use prisma::PrismaClient;
use tokio::sync::Mutex;
use crate::{entities::{AutovettureEntity, DipendentiEntity, ImpreseAssociateUtentiEntity, ImpreseCollegateEntity, ImpreseEntity, MansioneEntity, MezziEntity, OpereEntity, QualificaEntity, SettimanaleEntity, TipiProprietaEntity, UtentiEntity}, query_states::*, response_states::UtentiResponse, utils::*};



pub async fn get_settimanale(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetSettimanaleQuery>,
) -> Result<Json<Vec<SettimanaleEntity>>, APIError> {
    let client = prisma.lock().await;

    // Costruisce i filtri condizionali
    let mut filters = vec![];

    if let Some(data_settimanale_filter) = params.data_settimanale {
        filters.push(prisma::settimanale::WhereParam::DataSettimanale(
            prisma::read_filters::StringFilter::Equals(data_settimanale_filter),
        ));
    }

    if let Some(utente_id_filter) = params.utente_id {
        filters.push(prisma::settimanale::WhereParam::UtenteId(
            prisma::read_filters::IntFilter::Equals(utente_id_filter),
        ));
    }

    if let Some(luogo_di_nascita_filter) = params.luogo_di_nascita {
        filters.push(prisma::settimanale::WhereParam::LuogoDiNascita(
            prisma::read_filters::StringFilter::Contains(luogo_di_nascita_filter),
        ));
    }

    if let Some(data_di_nascita_filter) = params.data_di_nascita {
        filters.push(prisma::settimanale::WhereParam::DataDiNascita(
            prisma::read_filters::DateTimeFilter::Equals(
                data_di_nascita_filter.parse().map_err(|_| APIError {
                    message: "Invalid date format for data_di_nascita".to_string(),
                    status_code: StatusCode::BAD_REQUEST,
                    error_code: Some(400),
                })?,
            ),
        ));
    }

    if let Some(tipo_proprieta_filter) = params.tipo_proprieta {
        filters.push(prisma::settimanale::WhereParam::TipoProprieta(
            prisma::read_filters::IntFilter::Equals(tipo_proprieta_filter),
        ));
    }

    if let Some(proprieta_filter) = params.proprieta {
        filters.push(prisma::settimanale::WhereParam::Proprieta(
            prisma::read_filters::StringFilter::Contains(proprieta_filter),
        ));
    }

    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::settimanale::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }

    if let Some(opera_id_filter) = params.opera_id {
        filters.push(prisma::settimanale::WhereParam::OperaId(
            prisma::read_filters::IntFilter::Equals(opera_id_filter),
        ));
    }

    if let Some(mezzo_id_filter) = params.mezzo_id {
        filters.push(prisma::settimanale::WhereParam::MezzoId(
            prisma::read_filters::IntNullableFilter::Equals(Some(mezzo_id_filter)),
        ));
    }

    if let Some(autovettura_id_filter) = params.autovettura_id {
        filters.push(prisma::settimanale::WhereParam::AutovetturaId(
            prisma::read_filters::IntFilter::Equals(autovettura_id_filter),
        ));
    }

    if let Some(matricola_filter) = params.matricola {
        filters.push(prisma::settimanale::WhereParam::Matricola(
            prisma::read_filters::StringNullableFilter::Contains(matricola_filter),
        ));
    }

    if let Some(targa_filter) = params.targa {
        filters.push(prisma::settimanale::WhereParam::Targa(
            prisma::read_filters::StringFilter::Contains(targa_filter),
        ));
    }

    let results = client
        .settimanale()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_qualifiche(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetQualificheQuery>,
) -> Result<Json<Vec<QualificaEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::qualifica::WhereParam::Descrizione(
            prisma::read_filters::StringFilter::Contains(descrizione_filter),
        ));
    }

    let results = client
        .qualifica()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_qualifica(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetQualificaQuery>,
) -> Result<Json<QualificaEntity>, APIError> {
    let client = prisma.lock().await;

    let qualifica = client
        .qualifica()
        .find_first(vec![prisma::qualifica::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    qualifica.ok_or_else(|| APIError {
        message: "Dati Qualifica non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_mansioni(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetMansioniQuery>,
) -> Result<Json<Vec<MansioneEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::mansione::WhereParam::Descrizione(
            prisma::read_filters::StringFilter::Contains(descrizione_filter),
        ));
    }

    let results = client
        .mansione()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_mansione(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetMansioneQuery>,
) -> Result<Json<MansioneEntity>, APIError> {
    let client = prisma.lock().await;

    let mansione = client
        .mansione()
        .find_first(vec![prisma::mansione::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    mansione.ok_or_else(|| APIError {
        message: "Dati Mansione non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_opere(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetOpereQuery>,
) -> Result<Json<Vec<OpereEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::opere::WhereParam::Descrizione(
            prisma::read_filters::StringFilter::Contains(descrizione_filter),
        ));
    }

    let results = client
        .opere()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_opera(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetOperaQuery>,
) -> Result<Json<OpereEntity>, APIError> {
    let client = prisma.lock().await;

    let opera = client
        .opere()
        .find_first(vec![prisma::opere::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    opera.ok_or_else(|| APIError {
        message: "Dati Opera non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_tipi_proprieta(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetTipiProprietaQuery>,
) -> Result<Json<Vec<TipiProprietaEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::tipi_proprieta::WhereParam::Descrizione(
            prisma::read_filters::StringFilter::Contains(descrizione_filter),
        ));
    }

    let results = client
        .tipi_proprieta()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_tipo_proprieta(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetTipoProprietaQuery>,
) -> Result<Json<TipiProprietaEntity>, APIError> {
    let client = prisma.lock().await;

    let tipo_proprieta = client
        .tipi_proprieta()
        .find_first(vec![prisma::tipi_proprieta::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    tipo_proprieta.ok_or_else(|| APIError {
        message: "Dati Tipo Proprieta non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_imprese(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpreseQuery>,
) -> Result<Json<Vec<ImpreseEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(ragione_sociale_filter) = params.ragione_sociale {
        filters.push(prisma::imprese::WhereParam::RagioneSociale(
            prisma::read_filters::StringFilter::Contains(ragione_sociale_filter),
        ));
    }
    if let Some(indirizzo_filter) = params.indirizzo {
        filters.push(prisma::imprese::WhereParam::Indirizzo(
            prisma::read_filters::StringFilter::Contains(indirizzo_filter),
        ));
    }
    if let Some(partita_iva_filter) = params.partita_iva {
        filters.push(prisma::imprese::WhereParam::PartitaIva(
            prisma::read_filters::StringFilter::Contains(partita_iva_filter),
        ));
    }

    let results = client
        .imprese()
        .find_many(filters)
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_impresa(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpresaQuery>,
) -> Result<Json<ImpreseEntity>, APIError> {
    let client = prisma.lock().await;

    let impresa = client
        .imprese()
        .find_first(vec![prisma::imprese::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    impresa.ok_or_else(|| APIError {
        message: "Dati Impresa non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_imprese_collegate(
    session: ReadableSession,
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpreseCollegateQuery>,
) -> Result<Json<Vec<ImpreseEntity>>, APIError> {
    let client = prisma.lock().await;

    let id_impresa = match session.get::<i32>("impresa_id") {
        Some(user_id) => user_id,
        None => return Err(APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        }),
    };

    let mut filters = vec![];
    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::imprese_collegate::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }
    if let Some(ruolo_impresa_filter) = params.ruolo_impresa {
        filters.push(prisma::imprese_collegate::WhereParam::RuoloImpresa(
            prisma::read_filters::StringFilter::Contains(ruolo_impresa_filter),
        ));
    }

    let imprese_collegate = client
        .imprese_collegate()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    let imprese_ids: Vec<i32> = imprese_collegate
        .into_iter()
        .filter(|impresa| impresa.impresa_id == id_impresa)
        .map(|impresa| impresa.imprese_collegata_id)
        .collect();

    let result = client
        .imprese()
        .find_many(vec![prisma::imprese::WhereParam::Id(prisma::read_filters::IntFilter::InVec(imprese_ids))])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(result))
}



pub async fn get_impresa_collegata(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpresaCollegataQuery>,
) -> Result<Json<ImpreseCollegateEntity>, APIError> {
    let client = prisma.lock().await;

    let impresa_collegata = client
        .imprese_collegate()
        .find_first(vec![prisma::imprese_collegate::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    impresa_collegata.ok_or_else(|| APIError {
        message: "Dati Impresa Collegata non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_utenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetUtentiQuery>,
) -> Result<Json<Vec<UtentiResponse>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(username_filter) = params.username {
        filters.push(prisma::utenti::WhereParam::Username(
            prisma::read_filters::StringFilter::Contains(username_filter),
        ));
    }
    if let Some(nome_filter) = params.nome {
        filters.push(prisma::utenti::WhereParam::Nome(
            prisma::read_filters::StringNullableFilter::Contains(nome_filter),
        ));
    }
    if let Some(cognome_filter) = params.cognome {
        filters.push(prisma::utenti::WhereParam::Cognome(
            prisma::read_filters::StringNullableFilter::Contains(cognome_filter),
        ));
    }

    let utenti_results = client
        .utenti()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    let imprese_results = client
        .imprese()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    let mut list_users = Vec::new();
    for user in utenti_results {
        let ragione_sociale_impresa = imprese_results
            .iter()
            .find(|impresa| Some(impresa.id) == user.impresa_id)
            .map(|impresa| impresa.ragione_sociale.clone())
            .unwrap_or_default();
        
        list_users.push(UtentiResponse {
            id: user.id,
            impresa: ragione_sociale_impresa,
            nome: user.nome.unwrap(),
            cognome: user.cognome.unwrap(),
            username: user.username,
            revocato: user.autorizzazione.unwrap_or(false),
        });
    }

    if let Some(impresa_filter) = params.impresa {
        list_users = list_users
            .into_iter()
            .filter(|user| user.impresa.to_lowercase().contains(&impresa_filter))
            .collect();
    }

    Ok(Json(list_users))
}


pub async fn get_utente(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetUtenteQuery>,
) -> Result<Json<UtentiEntity>, APIError> {
    let client = prisma.lock().await;

    let utente = client
        .utenti()
        .find_first(vec![prisma::utenti::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    utente.ok_or_else(|| APIError {
        message: "Dati Utente non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_imprese_associate_utenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpreseAssociateUtentisQuery>,
) -> Result<Json<Vec<ImpreseEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(utente_id_filter) = params.utente_id {
        filters.push(prisma::imprese_associate_utenti::WhereParam::UtenteId(
            prisma::read_filters::IntFilter::Equals(utente_id_filter),
        ));
    }
    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::imprese_associate_utenti::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }

    let imprese_associate = client
        .imprese_associate_utenti()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    let imprese_ids: Vec<i32> = imprese_associate
        .into_iter()
        .filter(|impresa| impresa.utente_id == params.utente_id.unwrap())
        .map(|impresa| impresa.impresa_id)
        .collect();

    let result = client
        .imprese()
        .find_many(vec![prisma::imprese::WhereParam::Id(prisma::read_filters::IntFilter::InVec(imprese_ids))])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(result))
}


pub async fn get_impresa_associata_utente(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetImpresaAssiociataUtenteQuery>,
) -> Result<Json<ImpreseAssociateUtentiEntity>, APIError> {
    let client = prisma.lock().await;

    let impresa_associata_utente = client
        .imprese_associate_utenti()
        .find_first(vec![prisma::imprese_associate_utenti::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    impresa_associata_utente.ok_or_else(|| APIError {
        message: "Dati Impresa Associata Utente non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


async fn get_dipendenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetDipendentiQuery>,
) -> Result<Json<Vec<DipendentiEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(nome_filter) = params.nome {
        filters.push(prisma::dipendenti::WhereParam::Nome(
            prisma::read_filters::StringFilter::Contains(nome_filter),
        ));
    }
    if let Some(cognome_filter) = params.cognome {
        filters.push(prisma::dipendenti::WhereParam::Cognome(
            prisma::read_filters::StringFilter::Contains(cognome_filter),
        ));
    }
    if let Some(matricola_filter) = params.matricola {
        filters.push(prisma::dipendenti::WhereParam::Matricola(
            prisma::read_filters::StringNullableFilter::Contains(matricola_filter),
        ));
    }
    if let Some(data_di_nascita_filter) = params.data_di_nascita {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_di_nascita_filter = DateTime::from_naive_utc_and_offset(data_di_nascita_filter.and_time(t), FixedOffset::east(0));
        filters.push(prisma::dipendenti::WhereParam::DataDiNascita(
            prisma::read_filters::DateTimeFilter::Equals(data_di_nascita_filter),
        ));
    }
    if let Some(luogo_di_nascita_filter) = params.luogo_di_nascita {
        filters.push(prisma::dipendenti::WhereParam::LuogoDiNascita(
            prisma::read_filters::StringFilter::Contains(luogo_di_nascita_filter),
        ));
    }
    if let Some(codice_fiscale_filter) = params.codice_fiscale {
        filters.push(prisma::dipendenti::WhereParam::CodiceFiscale(
            prisma::read_filters::StringFilter::Contains(codice_fiscale_filter),
        ));
    }
    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::dipendenti::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }
    if let Some(qualifica_filter) = params.qualifica {
        filters.push(prisma::dipendenti::WhereParam::QualificaId(
            prisma::read_filters::IntFilter::Equals(qualifica_filter),
        ));
    }
    if let Some(mansione_filter) = params.mansione {
        filters.push(prisma::dipendenti::WhereParam::MansioneId(
            prisma::read_filters::IntFilter::Equals(mansione_filter),
        ));
    }
    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni_filter = DateTime::from_naive_utc_and_offset(data_dimissioni_filter.and_time(t), FixedOffset::east(0));
        filters.push(prisma::dipendenti::WhereParam::DataDimissioni(
            prisma::read_filters::DateTimeNullableFilter::Equals(Some(data_dimissioni_filter)),
        ));
    }
    if let Some(rfid1_filter) = params.rfid1 {
        filters.push(prisma::dipendenti::WhereParam::Rfid1(
            prisma::read_filters::StringNullableFilter::Contains(rfid1_filter),
        ));
    }
    if let Some(rfid2_filter) = params.rfid2 {
        filters.push(prisma::dipendenti::WhereParam::Rfid2(
            prisma::read_filters::StringNullableFilter::Contains(rfid2_filter),
        ));
    }

    let results = client
        .dipendenti()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}



pub async fn get_dipendente(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetDipendenteQuery>,
) -> Result<Json<DipendentiEntity>, APIError> {
    let client = prisma.lock().await;

    let dipendente = client
        .dipendenti()
        .find_first(vec![prisma::dipendenti::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    dipendente.ok_or_else(|| APIError {
        message: "Dati Dipendente non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_mezzi(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetMezziQuery>,
) -> Result<Json<Vec<MezziEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::mezzi::WhereParam::Descrizione(
            prisma::read_filters::StringNullableFilter::Contains(descrizione_filter),
        ));
    }
    if let Some(modello_filter) = params.modello {
        filters.push(prisma::mezzi::WhereParam::Modello(
            prisma::read_filters::StringFilter::Contains(modello_filter),
        ));
    }
    if let Some(tipo_proprieta_filter) = params.tipo_proprieta {
        filters.push(prisma::mezzi::WhereParam::TipoProprieta(
            prisma::read_filters::IntFilter::Equals(tipo_proprieta_filter),
        ));
    }
    if let Some(proprieta_filter) = params.proprieta {
        filters.push(prisma::mezzi::WhereParam::Proprieta(
            prisma::read_filters::StringFilter::Contains(proprieta_filter),
        ));
    }
    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::mezzi::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }
    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni_filter = DateTime::from_naive_utc_and_offset(data_dimissioni_filter.and_time(t), FixedOffset::east(0));
        filters.push(prisma::mezzi::WhereParam::DataDimissioni(
            prisma::read_filters::DateTimeFilter::Equals(data_dimissioni_filter),
        ));
    }
    if let Some(rfid1_filter) = params.rfid1 {
        filters.push(prisma::mezzi::WhereParam::Rfid1(
            prisma::read_filters::StringFilter::Contains(rfid1_filter),
        ));
    }
    if let Some(rfid2_filter) = params.rfid2 {
        filters.push(prisma::mezzi::WhereParam::Rfid2(
            prisma::read_filters::StringFilter::Contains(rfid2_filter),
        ));
    }

    let results = client
        .mezzi()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_mezzo(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetMezzoQuery>,
) -> Result<Json<MezziEntity>, APIError> {
    let client = prisma.lock().await;

    let mezzo = client
        .mezzi()
        .find_first(vec![prisma::mezzi::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    mezzo.ok_or_else(|| APIError {
        message: "Dati Mezzo non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}


pub async fn get_autovetture(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetAutovettureQuery>,
) -> Result<Json<Vec<AutovettureEntity>>, APIError> {
    let client = prisma.lock().await;

    let mut filters = vec![];
    if let Some(descrizione_filter) = params.descrizione {
        filters.push(prisma::autovetture::WhereParam::Descrizione(
            prisma::read_filters::StringNullableFilter::Contains(descrizione_filter),
        ));
    }
    if let Some(modello_filter) = params.modello {
        filters.push(prisma::autovetture::WhereParam::Modello(
            prisma::read_filters::StringFilter::Contains(modello_filter),
        ));
    }
    if let Some(targa_filter) = params.targa {
        filters.push(prisma::autovetture::WhereParam::Targa(
            prisma::read_filters::StringFilter::Contains(targa_filter),
        ));
    }
    if let Some(tipo_proprieta_filter) = params.tipo_proprieta {
        filters.push(prisma::autovetture::WhereParam::TipoProprieta(
            prisma::read_filters::IntFilter::Equals(tipo_proprieta_filter),
        ));
    }
    if let Some(proprieta_filter) = params.proprieta {
        filters.push(prisma::autovetture::WhereParam::Proprieta(
            prisma::read_filters::StringFilter::Contains(proprieta_filter),
        ));
    }
    if let Some(impresa_id_filter) = params.impresa_id {
        filters.push(prisma::autovetture::WhereParam::ImpresaId(
            prisma::read_filters::IntFilter::Equals(impresa_id_filter),
        ));
    }
    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        let t = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let data_dimissioni_filter = DateTime::from_naive_utc_and_offset(data_dimissioni_filter.and_time(t), FixedOffset::east(0));
        filters.push(prisma::autovetture::WhereParam::DataDimissioni(
            prisma::read_filters::DateTimeFilter::Equals(data_dimissioni_filter),
        ));
    }
    if let Some(rfid1_filter) = params.rfid1 {
        filters.push(prisma::autovetture::WhereParam::Rfid1(
            prisma::read_filters::StringFilter::Contains(rfid1_filter),
        ));
    }
    if let Some(rfid2_filter) = params.rfid2 {
        filters.push(prisma::autovetture::WhereParam::Rfid2(
            prisma::read_filters::StringFilter::Contains(rfid2_filter),
        ));
    }

    let results = client
        .autovetture()
        .find_many(filters)
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(500),
        })?;

    Ok(Json(results))
}


pub async fn get_autovettura(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<GetAutovetturaQuery>,
) -> Result<Json<AutovettureEntity>, APIError> {
    let client = prisma.lock().await;

    let autovettura = client
        .autovetture()
        .find_first(vec![prisma::autovetture::WhereParam::Id(
            prisma::read_filters::IntFilter::Equals(params.id),
        )])
        .exec()
        .await
        .map_err(|e| APIError {
            message: e.to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(404),
        })?;

    autovettura.ok_or_else(|| APIError {
        message: "Dati Autovettura non trovati".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(404),
    }).map(Json)
}

