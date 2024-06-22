use std::sync::Arc;

use axum::{extract::{Query, State}, Json};
use axum_sessions::extractors::ReadableSession;
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use crate::{models::{self, Imprese}, query_states::*, response_states::UtentiResponse, schema, utils::*};


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/*pub fn get_qualifiche(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetQualificheQuery>,
) -> Result<Json<Vec<models::Qualifiche>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::qualifiches::dsl::*;

    /*let mut query_builder = qualifiches.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }*/

    let results = qualifiches
        .load::<models::Qualifiche>(&mut conn)
        .map_err(|e| format!("Failed to load qualifica: {}", e))?;

    Ok(Json(results))
}*/

pub async fn get_qualifiche(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetQualificheQuery>,
) -> Result<Json<Vec<models::Qualifiche>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::qualifiches::dsl::*;

    let mut query_builder = qualifiches.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    let results = query_builder
        .load::<models::Qualifiche>(&mut conn)
        .map_err(|e| format!("Failed to load qualifiche: {}", e))?;

    Ok(Json(results))
}

pub async fn get_qualifica(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetQualificaQuery>,
) -> Result<Json<models::Qualifiche>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::qualifiches::dsl::*;

    let results = qualifiches.filter(id.eq(params.id))
        .first::<models::Qualifiche>(&mut conn)
        .map_err(|e| format!("Failed to load qualifiche: {}", e))?;

    Ok(Json(results))
}

pub async fn get_mansioni(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetMansioniQuery>,
) -> Result<Json<Vec<models::Mansioni>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mansionis::dsl::*;

    let mut query_builder = mansionis.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    let results = query_builder
        .load::<models::Mansioni>(&mut conn)
        .map_err(|e| format!("Failed to load mansione: {}", e))?;

    Ok(Json(results))
}

pub async fn get_mansione(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetMansioneQuery>,
) -> Result<Json<models::Mansioni>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mansionis::dsl::*;
    

    let results = mansionis.filter(id.eq(params.id))
        .first::<models::Mansioni>(&mut conn)
        .map_err(|e| format!("Failed to load mansione: {}", e))?;

    Ok(Json(results))
}

pub async fn get_opere(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetOpereQuery>,
) -> Result<Json<Vec<models::Opere>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::operes::dsl::*;

    let mut query_builder = operes.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    let results = query_builder
        .load::<models::Opere>(&mut conn)
        .map_err(|e| format!("Failed to load opera: {}", e))?;

    Ok(Json(results))
}

pub async fn get_opera(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetOperaQuery>,
) -> Result<Json<models::Opere>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::operes::dsl::*;

    let results = operes.filter(id.eq(params.id))
        .first::<models::Opere>(&mut conn)
        .map_err(|e| format!("Failed to load opera: {}", e))?;

    Ok(Json(results))
}

pub async fn get_tipi_proprieta(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetTipiProprietaQuery>,
) -> Result<Json<Vec<models::TipiProprieta>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::tipi_proprietas::dsl::*;

    let mut query_builder = tipi_proprietas.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    let results = query_builder
        .load::<models::TipiProprieta>(&mut conn)
        .map_err(|e| format!("Failed to load tipo proprieta: {}", e))?;

    Ok(Json(results))
}

pub async fn get_tipo_proprieta(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetTipoProprietaQuery>,
) -> Result<Json<models::TipiProprieta>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::tipi_proprietas::dsl::*;

    let results = tipi_proprietas.filter(id.eq(params.id))
        .first::<models::TipiProprieta>(&mut conn)
        .map_err(|e| format!("Failed to load tipo proprieta: {}", e))?;

    Ok(Json(results))
}

pub async fn get_imprese(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpreseQuery>,
) -> Result<Json<Vec<models::Imprese>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::impreses::dsl::*;

    let mut query_builder = impreses.into_boxed();

    if let Some(ragione_sociale_filter) = params.ragione_sociale {
        query_builder = query_builder.filter(ragione_sociale.like(format!("%{}%", ragione_sociale_filter)));
    }

    if let Some(indirizzo_filter) = params.indirizzo {
        query_builder = query_builder.filter(indirizzo.like(format!("%{}%", indirizzo_filter)));
    }

    if let Some(partita_iva_filter) = params.partita_iva {
        query_builder = query_builder.filter(partita_iva.like(format!("%{}%", partita_iva_filter)));
    }

    let results = query_builder
        .load::<models::Imprese>(&mut conn)
        .map_err(|e| format!("Failed to load impresa: {}", e))?;

    Ok(Json(results))
}

pub async fn get_impresa(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpresaQuery>,
) -> Result<Json<models::Imprese>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::impreses::dsl::*;

    let results = impreses.filter(id.eq(params.id))
        .first::<models::Imprese>(&mut conn)
        .map_err(|e| format!("Failed to load impresa: {}", e))?;

    Ok(Json(results))
}

pub async fn get_imprese_collegate(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpreseCollegateQuery>,
) -> Result<Json<Vec<models::Imprese>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::imprese_collegates::dsl::*;
    use schema::impreses::dsl::*;

    let mut query_builder = imprese_collegates.into_boxed();

    if let Some(impresa_id_filter) = params.impresa_id {
        query_builder = query_builder.filter(impresa_id.eq(impresa_id_filter));
    }

    if let Some(ruolo_impresa_filter) = params.ruolo_impresa {
        query_builder = query_builder.filter(ruolo_impresa.like(format!("%{}%", ruolo_impresa_filter)));
    }

    let results = query_builder
        .load::<models::ImpreseCollegate>(&mut conn)
        .map_err(|e| format!("Failed to load impresa collegata: {}", e))?;

    let id_impresa = match session.get::<i32>("impresa_id"){
            Some(user_id) => user_id,
            None => return Err("Unauthorized".to_string())};


    let imprese_ids = results.clone().into_iter().filter(|impresa| impresa.impresa_id == params.impresa_id.unwrap()).map(|impresa| impresa.imprese_collegata_id).collect::<Vec<i32>>();

    let result: Vec<models::Imprese> = impreses
    .select(models::Imprese::as_select())
    .filter(schema::impreses::dsl::id.eq_any(imprese_ids))
    .load::<models::Imprese>(&mut conn)
    .map_err(|e| format!("Failed to load imprese: {}", e))?;

    Ok(Json(result))
}


pub async fn get_impresa_collegata(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpresaCollegataQuery>,
) -> Result<Json<models::ImpreseCollegate>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::imprese_collegates::dsl::*;

    let results = imprese_collegates.filter(id.eq(params.id))
        .first::<models::ImpreseCollegate>(&mut conn)
        .map_err(|e| format!("Failed to load impresa collegata: {}", e))?;

    Ok(Json(results))
}

pub async fn get_utenti(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetUtentiQuery>,
) -> Result<Json<Vec<UtentiResponse>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::utentis::dsl::*;
    use schema::impreses::dsl::*;

    let mut list_users: Vec<UtentiResponse> = vec![];

    let mut query_builder = utentis.into_boxed();

    if let Some(username_filter) = params.username {
        query_builder = query_builder.filter(username.like(format!("%{}%", username_filter)));
    }

    if let Some(nome_filter) = params.nome {
        query_builder = query_builder.filter(nome.like(format!("%{}%", nome_filter)));
    }

    if let Some(cognome_filter) = params.cognome {
        query_builder = query_builder.filter(cognome.like(format!("%{}%", cognome_filter)));
    }

    let utenti_results = query_builder
        .load::<models::Utenti>(&mut conn)
        .map_err(|e| format!("Failed to load utente: {}", e))?;

    let imprese_results = impreses
        .load::<models::Imprese>(&mut conn)
        .map_err(|e| format!("Failed to load utente: {}", e))?;

    for user in utenti_results {
        let ragione_sociale_impresa = imprese_results.iter()
        .find(|impresa| impresa.id == user.impresa_id)
        .map(|impresa| impresa.ragione_sociale.clone())
        .unwrap_or_default();
        list_users.push(UtentiResponse {
            id: user.id,
            impresa: ragione_sociale_impresa.to_string(),
            nome: user.nome,
            cognome: user.cognome,
            username: user.username,
            revocato: user.autorizazzione.unwrap(),
        })
    }


    if let Some(impresa_filter) = params.impresa {
        list_users = list_users.into_iter().filter(|user| user.impresa.to_lowercase().contains(&impresa_filter)).collect();
    }

    Ok(Json(list_users))
}

pub async fn get_utente(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetUtenteQuery>,
) -> Result<Json<models::Utenti>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::utentis::dsl::*;

    let results = utentis.filter(id.eq(params.id))
        .first::<models::Utenti>(&mut conn)
        .map_err(|e| format!("Failed to load utente: {}", e))?;

    Ok(Json(results))
}

pub async fn get_imprese_associate_utenti(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpreseAssociateUtentisQuery>,
) -> Result<Json<Vec<models::Imprese>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::imprese_associate_utentis::dsl::*;
    use schema::impreses::dsl::*;

    let mut query_builder = imprese_associate_utentis.into_boxed();

    if let Some(utente_id_filter) = params.utente_id {
        query_builder = query_builder.filter(utente_id.eq(utente_id_filter));
    }

    if let Some(impresa_id_filter) = params.impresa_id {
        query_builder = query_builder.filter(impresa_id.eq(impresa_id_filter));
    }

    let results = query_builder
        .load::<models::ImpreseAssociateUtenti>(&mut conn)
        .map_err(|e| format!("Failed to load impresa associata utente: {}", e))?;

    let imprese_ids = results.clone().into_iter().filter(|impresa| impresa.utente_id == params.utente_id.unwrap()).map(|impresa| impresa.impresa_id).collect::<Vec<i32>>();

    let result: Vec<models::Imprese> = impreses
    .select(models::Imprese::as_select())
    .filter(schema::impreses::dsl::id.eq_any(imprese_ids))
    .load::<models::Imprese>(&mut conn)
    .map_err(|e| format!("Failed to load imprese: {}", e))?;


    Ok(Json(result))
}

pub async fn get_impresa_associata_utente(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetImpresaAssiociataUtenteQuery>,
) -> Result<Json<models::ImpreseAssociateUtenti>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::imprese_associate_utentis::dsl::*;

    let results = imprese_associate_utentis.filter(id.eq(params.id))
        .first::<models::ImpreseAssociateUtenti>(&mut conn)
        .map_err(|e| format!("Failed to load impresa associata utente: {}", e))?;

    Ok(Json(results))
}

pub async fn get_dipendenti(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetDipendentiQuery>,
) -> Result<Json<Vec<models::Dipendenti>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::dipendentis::dsl::*;

    let mut query_builder = dipendentis.into_boxed();

    if let Some(nome_filter) = params.nome {
        query_builder = query_builder.filter(nome.like(format!("%{}%", nome_filter)));
    }

    if let Some(cognome_filter) = params.cognome {
        query_builder = query_builder.filter(cognome.like(format!("%{}%", cognome_filter)));
    }

    if let Some(matricola_filter) = params.matricola {
        query_builder = query_builder.filter(matricola.like(format!("%{}%", matricola_filter)));
    }

    if let Some(data_di_nascita_filter) = params.data_di_nascita {
        query_builder = query_builder.filter(data_di_nascita.eq(data_di_nascita_filter));
    }

    if let Some(luogo_di_nascita_filter) = params.luogo_di_nascita {
        query_builder = query_builder.filter(luogo_di_nascita.like(format!("%{}%", luogo_di_nascita_filter)));
    }

    if let Some(codice_fiscale_filter) = params.codice_fiscale {
        query_builder = query_builder.filter(codice_fiscale.like(format!("%{}%", codice_fiscale_filter)));
    }

    if let Some(impresa_id_filter) = params.impresa_id {
        query_builder = query_builder.filter(impresa_id.eq(impresa_id_filter));
    }

    if let Some(qualifica_filter) = params.qualifica {
        query_builder = query_builder.filter(qualifica.eq(qualifica_filter));
    }

    if let Some(mansione_filter) = params.mansione {
        query_builder = query_builder.filter(mansione.eq(mansione_filter));
    }

    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        query_builder = query_builder.filter(data_dimissioni.eq(data_dimissioni_filter));
    }

    if let Some(rfid1_filter) = params.rfid1 {
        query_builder = query_builder.filter(rfid1.like(format!("%{}%", rfid1_filter)));
    }

    if let Some(rfid2_filter) = params.rfid2 {
        query_builder = query_builder.filter(rfid2.like(format!("%{}%", rfid2_filter)));
    }

    let results = query_builder
        .load::<models::Dipendenti>(&mut conn)
        .map_err(|e| format!("Failed to load dipendente: {}", e))?;

    Ok(Json(results))
}


pub async fn get_dipendente(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetDipendenteQuery>,
) -> Result<Json<models::Dipendenti>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::dipendentis::dsl::*;

    let results = dipendentis.filter(id.eq(params.id))
        .first::<models::Dipendenti>(&mut conn)
        .map_err(|e| format!("Failed to load dipendente: {}", e))?;

    Ok(Json(results))
}

pub async fn get_mezzi(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetMezziQuery>,
) -> Result<Json<Vec<models::Mezzi>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mezzis::dsl::*;

    let mut query_builder = mezzis.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    if let Some(modello_filter) = params.modello {
        query_builder = query_builder.filter(modello.like(format!("%{}%", modello_filter)));
    }

    if let Some(tipo_proprieta_filter) = params.tipo_proprieta {
        query_builder = query_builder.filter(tipo_proprieta.eq(tipo_proprieta_filter));
    }

    if let Some(proprieta_filter) = params.proprieta {
        query_builder = query_builder.filter(proprieta.like(format!("%{}%", proprieta_filter)));
    }

    if let Some(impresa_id_filter) = params.impresa_id {
        query_builder = query_builder.filter(impresa_id.eq(impresa_id_filter));
    }

    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        query_builder = query_builder.filter(data_dimissioni.eq(data_dimissioni_filter));
    }

    if let Some(rfid1_filter) = params.rfid1 {
        query_builder = query_builder.filter(rfid1.like(format!("%{}%", rfid1_filter)));
    }

    if let Some(rfid2_filter) = params.rfid2 {
        query_builder = query_builder.filter(rfid2.like(format!("%{}%", rfid2_filter)));
    }

    let results = query_builder
        .load::<models::Mezzi>(&mut conn)
        .map_err(|e| format!("Failed to load mezzo: {}", e))?;

    Ok(Json(results))
}

pub async fn get_mezzo(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetMezzoQuery>,
) -> Result<Json<models::Mezzi>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mezzis::dsl::*;

    let results = mezzis.filter(id.eq(params.id))
        .first::<models::Mezzi>(&mut conn)
        .map_err(|e| format!("Failed to load mezzo: {}", e))?;

    Ok(Json(results))
}

pub async fn get_autovetture(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetAutovettureQuery>,
) -> Result<Json<Vec<models::Autovetture>>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::autovettures::dsl::*;

    let mut query_builder = autovettures.into_boxed();

    if let Some(descrizione_filter) = params.descrizione {
        query_builder = query_builder.filter(descrizione.like(format!("%{}%", descrizione_filter)));
    }

    if let Some(modello_filter) = params.modello {
        query_builder = query_builder.filter(modello.like(format!("%{}%", modello_filter)));
    }

    if let Some(targa_filter) = params.targa {
        query_builder = query_builder.filter(targa.like(format!("%{}%", targa_filter)));
    }

    if let Some(tipo_proprieta_filter) = params.tipo_proprieta {
        query_builder = query_builder.filter(tipo_proprieta.eq(tipo_proprieta_filter));
    }

    if let Some(proprieta_filter) = params.proprieta {
        query_builder = query_builder.filter(proprieta.like(format!("%{}%", proprieta_filter)));
    }

    if let Some(impresa_id_filter) = params.impresa_id {
        query_builder = query_builder.filter(impresa_id.eq(impresa_id_filter));
    }

    if let Some(data_dimissioni_filter) = params.data_dimissioni {
        query_builder = query_builder.filter(data_dimissioni.eq(data_dimissioni_filter));
    }

    if let Some(rfid1_filter) = params.rfid1 {
        query_builder = query_builder.filter(rfid1.like(format!("%{}%", rfid1_filter)));
    }

    if let Some(rfid2_filter) = params.rfid2 {
        query_builder = query_builder.filter(rfid2.like(format!("%{}%", rfid2_filter)));
    }

    let results = query_builder
        .load::<models::Autovetture>(&mut conn)
        .map_err(|e| format!("Failed to load autovettura: {}", e))?;

    Ok(Json(results))
}

pub async fn get_autovettura(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<GetAutovetturaQuery>,
) -> Result<Json<models::Autovetture>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::autovettures::dsl::*;

    let results = autovettures.filter(id.eq(params.id))
        .first::<models::Autovetture>(&mut conn)
        .map_err(|e| format!("Failed to load autovettura: {}", e))?;

    Ok(Json(results))
}
