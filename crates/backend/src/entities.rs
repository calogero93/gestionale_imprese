pub type AutovettureEntity = prisma::autovetture::Data;
pub type DipendentiEntity = prisma::dipendenti::Data;
pub type ImpreseAssociateUtentiEntity = prisma::imprese_associate_utenti::Data;
pub type ImpreseCollegateEntity = prisma::imprese_collegate::Data;
pub type ImpreseEntity = prisma::imprese::Data;
pub type MansioneEntity = prisma::mansione::Data;
pub type MezziEntity = prisma::mezzi::Data;
pub type OpereEntity = prisma::opere::Data;
pub type QualificaEntity = prisma::qualifica::Data;
pub type SettimanaleEntity = prisma::settimanale::Data;
pub type TipiProprietaEntity = prisma::tipi_proprieta::Data;
pub type UtentiEntity = prisma::utenti::Data;

pub struct EntityPage<T: prisma_client_rust::Data> {
    pub page: Vec<T>,
    pub total_count: i64,
}