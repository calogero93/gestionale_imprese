use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use crate::{models, request_states::*, schema, utils::*};


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn update_qualifiche(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateQualificheRequest>,
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
    use schema::qualifiches::dsl::*;

    let target = qualifiches.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(
            &payload.0
        )
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update qualifica: {}", e))?;

    Ok(Json("Qualifica aggiornata con successo").into_response())
}

pub async fn update_mansioni(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateMansioniRequest>,
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
    use schema::mansionis::dsl::*;

    let target = mansionis.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update mansione: {}", e))?;

    Ok(Json("Mansione aggiornata con successo").into_response())
}

pub async fn update_opere(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateOpereRequest>,
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
    use schema::operes::dsl::*;

    let target = operes.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update opera: {}", e))?;

    Ok(Json("Opera aggiornata con successo").into_response())
}

pub async fn update_tipi_proprieta(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateTipiProprietaRequest>,
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
    use schema::tipi_proprietas::dsl::*;

    let target = tipi_proprietas.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update tipo_proprieta: {}", e))?;

    Ok(Json("Tipo Proprieta aggiornato con successo").into_response())
}

pub async fn update_imprese(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateImpreseRequest>,
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
    use schema::impreses::dsl::*;

    let target = impreses.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update impresa: {}", e))?;

    Ok(Json("Impresa aggiornata con successo").into_response())
}

pub async fn update_imprese_collegate(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateImpreseCollegateRequest>,
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
    use schema::imprese_collegates::dsl::*;

    let target = imprese_collegates.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update impresa_collegata: {}", e))?;

    Ok(Json("Impresa collegata aggiornata con successo").into_response())
}

pub async fn update_utenti(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateUtentiRequest>,
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
    use schema::utentis::dsl::*;

    let target = utentis.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update utente: {}", e))?;

    Ok(Json("Utente aggiornato con successo").into_response())
}

pub async fn update_imprese_associate_utentis(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    payload: Json<UpdateImpreseAssociateUtentisRequest>,
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
    use schema::imprese_associate_utentis::dsl::*;

    let target = imprese_associate_utentis.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update impresa_associata_utente: {}", e))?;

    Ok(Json("Impresa associata all'utente aggiornata con successo").into_response())
}

pub async fn update_dipendenti(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    mut payload: Json<UpdateDipendentiRequest>,
) -> anyhow::Result<impl IntoResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

    use schema::dipendentis::dsl::*;

    let super_user = match session.get::<bool>("super_utente"){
        Some(super_user) => super_user,
        None => false
    };

    let user_id = match session.get::<String>("user_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let id_impresa = match session.get::<String>("impresa_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let user_id: i32 = user_id.parse().unwrap();
    let id_impresa: i32 = id_impresa.parse().unwrap();
    let utente = get_utente(&mut conn, payload.id)?;

    let imprese_associate = get_imprese_associate_utenti(&mut conn, user_id)?;

    let imprese_associate_id = imprese_associate.into_iter().map(|impresa| impresa.impresa_id).collect::<Vec<i32>>();

    if !(id_impresa == utente.impresa_id || imprese_associate_id.contains(&id_impresa) || super_user) {
        return Err("Unauthorized".to_string())
    }

    if !super_user {
        payload.0.impresa_id = None
    }

    let target = dipendentis.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update dipendente: {}", e))?;

    Ok(Json("Dipendente aggiornato con successo").into_response())
}

pub async fn update_mezzi(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    mut payload: Json<UpdateMezziRequest>,
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

    let id_impresa = match session.get::<String>("impresa_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let user_id: i32 = user_id.parse().unwrap();
    let id_impresa: i32 = id_impresa.parse().unwrap();
    let mezzo = get_mezzo(&mut conn, payload.id)?;

    let imprese_associate = get_imprese_associate_utenti(&mut conn, user_id)?;

    let imprese_associate_id = imprese_associate.into_iter().map(|impresa| impresa.impresa_id).collect::<Vec<i32>>();

    if !(id_impresa == mezzo.impresa_id || imprese_associate_id.contains(&id_impresa) || super_user) {
        return Err("Unauthorized".to_string())
    }

    if !super_user {
        payload.0.impresa_id = None
    }

    use schema::mezzis::dsl::*;

    let target = mezzis.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update mezzo: {}", e))?;

    Ok(Json("Mezzo aggiornato con successo").into_response())
}

pub async fn update_autovetture(
    session: ReadableSession,
    State(pool): State<Arc<DbPool>>,
    mut payload: Json<UpdateAutovettureRequest>,
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

    let id_impresa = match session.get::<String>("impresa_id"){
        Some(user_id) => user_id,
        None => return Err("Unauthorized".to_string())
    };

    let user_id: i32 = user_id.parse().unwrap();
    let id_impresa: i32 = id_impresa.parse().unwrap();
    let autevettura = get_autovettura(&mut conn, payload.id)?;

    let imprese_associate = get_imprese_associate_utenti(&mut conn, user_id)?;

    let imprese_associate_id = imprese_associate.into_iter().map(|impresa| impresa.impresa_id).collect::<Vec<i32>>();

    if !(id_impresa == autevettura.impresa_id || imprese_associate_id.contains(&id_impresa) || super_user) {
        return Err("Unauthorized".to_string())
    }

    if !super_user {
        payload.0.impresa_id = None
    }

    use schema::autovettures::dsl::*;

    let target = autovettures.filter(id.eq(payload.id));
    let changes = diesel::update(target)
        .set(&payload.0)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to update autovettura: {}", e))?;

    Ok(Json("Autovettura aggiornata con successo").into_response())
}

