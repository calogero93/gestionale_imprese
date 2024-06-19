use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use crate::{models, remove_state::*, schema, utils::*};


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn remove_autovettura(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveAutovetturaQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::autovettures::dsl::*;
    
    let num_deleted = diesel::delete(autovettures.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete autovettura"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} autovettura", num_deleted)))
    } else {
        Err(format!("No autovettura found with ID {}", params.id))
    }
}

pub async fn remove_dipendente(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveDipendenteQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::dipendentis::dsl::*;
    let num_deleted = diesel::delete(dipendentis.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete dipendente"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} dipendente", num_deleted)))
    } else {
        Err(format!("No dipendente found with ID {}", params.id))
    }
}

pub async fn remove_impresa(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveImpresaQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::impreses::dsl::*;
    let num_deleted = diesel::delete(impreses.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete impresa"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} impresa", num_deleted)))
    } else {
        Err(format!("No impresa found with ID {}", params.id))
    }
}

pub async fn remove_imprese_associate_utenti(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveImpresaAssociateUtenteQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::imprese_associate_utentis::dsl::*;
    let num_deleted = diesel::delete(imprese_associate_utentis.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete imprese associate utenti"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} imprese associate utenti", num_deleted)))
    } else {
        Err(format!("No imprese associate utenti found with ID {}", params.id))
    }
}

pub async fn remove_mansione(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveMansioneQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mansionis::dsl::*;
    let num_deleted = diesel::delete(mansionis.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete mansione"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} mansione", num_deleted)))
    } else {
        Err(format!("No mansione found with ID {}", params.id))
    }
}

pub async fn remove_mezzo(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveMezzoQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::mezzis::dsl::*;
    let num_deleted = diesel::delete(mezzis.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete mezzo"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} mezzo", num_deleted)))
    } else {
        Err(format!("No mezzo found with ID {}", params.id))
    }
}

pub async fn remove_opera(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveOperaQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::operes::dsl::*;
    let num_deleted = diesel::delete(operes.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete opera"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} opera", num_deleted)))
    } else {
        Err(format!("No opera found with ID {}", params.id))
    }
}

pub async fn remove_qualifica(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveQualificaQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::qualifiches::dsl::*;
    let num_deleted = diesel::delete(qualifiches.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete qualifica"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} qualifica", num_deleted)))
    } else {
        Err(format!("No qualifica found with ID {}", params.id))
    }
}

pub async fn remove_tipo_proprieta(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveTipoProprietaQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::tipi_proprietas::dsl::*;
    let num_deleted = diesel::delete(tipi_proprietas.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete tipo proprieta"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} tipo proprieta", num_deleted)))
    } else {
        Err(format!("No tipo proprieta found with ID {}", params.id))
    }
}

pub async fn remove_utente(
    State(pool): State<Arc<DbPool>>,
    Query(params): Query<RemoveUtenteQuery>,
) -> Result<Json<String>, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::utentis::dsl::*;
    let num_deleted = diesel::delete(utentis.filter(
        id.eq(params.id)
    ))
    .execute(&mut conn)
    .map_err(handle_diesel_error("Failed to delete utente"))?;

    if num_deleted > 0 {
        Ok(Json(format!("Deleted {} utente", num_deleted)))
    } else {
        Err(format!("No utente found with ID {}", params.id))
    }
}

fn handle_diesel_error(custom_message: &'static str) -> impl Fn(diesel::result::Error) -> String {
    move |e| {
        println!("Diesel Error: {:?}", e);
        custom_message.to_string()
    }
}
