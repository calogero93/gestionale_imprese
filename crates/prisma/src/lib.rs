#[rustfmt::skip]
mod generated;

use std::{ops::Deref, sync::Arc};

pub use generated::*;
use prisma_client_rust::NewClientError;

#[derive(Debug)]
pub enum PrismaClientError {
    NewClientError(NewClientError),
}

#[derive(Clone)]
pub struct PrismaClient(Arc<generated::_prisma::PrismaClient>);

impl Deref for PrismaClient {
    type Target = _prisma::PrismaClient;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub async fn instance_prisma_client(mysql_connection: &str) -> Result<PrismaClient, PrismaClientError> {
    Ok(PrismaClient(Arc::new(
        _prisma::PrismaClient::_builder()
            .with_url(mysql_connection.to_string())
            .build()
            .await
            .map_err(PrismaClientError::NewClientError)?,
    )))
}