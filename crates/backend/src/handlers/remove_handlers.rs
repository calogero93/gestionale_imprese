use std::sync::Arc;

use axum::{extract::{Query, State}, response::IntoResponse, Json};
use axum_sessions::extractors::ReadableSession;
use diesel::{r2d2::{self, ConnectionManager}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use hyper::StatusCode;
use prisma::PrismaClient;
use tokio::sync::Mutex;
use crate::{models, remove_state::*, schema::{self, imprese_collegates}, utils::*};


pub async fn remove_autovettura(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveAutovetturaQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.autovetture()
        .delete(prisma::autovetture::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete autovettura: {}", e))?;

    Ok((StatusCode::OK, Json("Autovettura removed successfully")))
}


pub async fn remove_dipendente(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveDipendenteQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.dipendenti()
        .delete(prisma::dipendenti::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete dipendente: {}", e))?;

    Ok((StatusCode::OK, Json("Dipendente removed successfully")))
}


pub async fn remove_impresa(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveImpresaQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.imprese()
        .delete(prisma::imprese::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete impresa: {}", e))?;

    Ok((StatusCode::OK, Json("Impresa removed successfully")))
}


pub async fn remove_imprese_associate_utenti(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveImpresaAssociateUtenteQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.imprese_associate_utenti()
        .delete(prisma::imprese_associate_utenti::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete imprese associate utenti: {}", e))?;

    Ok((StatusCode::OK, Json("Imprese associate utenti removed successfully")))
}


pub async fn remove_imprese_collegate(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveImpresaCollegataQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.imprese_collegate()
        .delete(prisma::imprese_collegate::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete imprese collegate: {}", e))?;

    Ok((StatusCode::OK, Json("Imprese collegate removed successfully")))
}


pub async fn remove_mansione(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveMansioneQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.mansione()
        .delete(prisma::mansione::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete mansione: {}", e))?;

    Ok((StatusCode::OK, Json("Mansione removed successfully")))
}


pub async fn remove_mezzo(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveMezzoQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.mezzi()
        .delete(prisma::mezzi::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete mezzo: {}", e))?;

    Ok((StatusCode::OK, Json("Mezzo removed successfully")))
}


pub async fn remove_opera(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveOperaQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.opere()
        .delete(prisma::opere::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete opera: {}", e))?;

    Ok((StatusCode::OK, Json("Opera removed successfully")))
}


pub async fn remove_qualifica(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveQualificaQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.qualifica()
        .delete(prisma::qualifica::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete qualifica: {}", e))?;

    Ok((StatusCode::OK, Json("Qualifica removed successfully")))
}


pub async fn remove_tipo_proprieta(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveTipoProprietaQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.tipi_proprieta()
        .delete(prisma::tipi_proprieta::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete tipo proprieta: {}", e))?;

    Ok((StatusCode::OK, Json("Tipo proprieta removed successfully")))
}


pub async fn remove_utente(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveUtenteQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.utenti()
        .delete(prisma::utenti::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete utente: {}", e))?;

    Ok((StatusCode::OK, Json("Utente removed successfully")))
}


pub async fn remove_settimanale(
    State(prisma): State<Arc<Mutex<PrismaClient>>>,
    Query(params): Query<RemoveSettimanaleQuery>,
) -> Result<impl IntoResponse, String> {
    let client = prisma.lock().await;

    client.settimanale()
        .delete(prisma::settimanale::UniqueWhereParam::IdEquals(params.id))
        .exec()
        .await
        .map_err(|e| format!("Failed to delete settimanale: {}", e))?;

    Ok((StatusCode::OK, Json("Settimanale removed successfully")))
}

