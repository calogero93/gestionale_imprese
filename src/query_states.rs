use axum_sessions::async_session::chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetQualificheQuery {
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct GetQualificaQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct GetMansioniQuery {
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct GetOpereQuery {
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct GetTipiProprietaQuery {
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct GetImpreseQuery {
    pub ragione_sociale: Option<String>,
    pub indirizzo: Option<String>,
    pub partita_iva: Option<String>,
}

#[derive(Deserialize)]
pub struct GetImpreseCollegateQuery {
    pub impresa_id: Option<i32>,
    pub ruolo_impresa: Option<String>,
}

#[derive(Deserialize)]
pub struct GetUtentiQuery {
    pub username: Option<String>,
    pub nome: Option<String>,
    pub cognome: Option<String>,
}

#[derive(Deserialize)]
pub struct GetImpreseAssociateUtentisQuery {
    pub utente_id: Option<i32>,
    pub impresa_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct GetDipendentiQuery {
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub matricola: Option<String>,
    pub data_di_nascita: Option<NaiveDate>,
    pub luogo_di_nascita: Option<String>,
    pub codice_fiscale: Option<String>,
    pub impresa_id: Option<i32>,
    pub qualifica: Option<i32>,
    pub mansione: Option<i32>,
    pub data_dimissioni: Option<NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}

#[derive(Deserialize)]
pub struct GetMezziQuery {
    pub descrizione: Option<String>,
    pub modello: Option<String>,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: Option<String>,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: Option<NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}

#[derive(Deserialize)]
pub struct GetAutovettureQuery {
    pub descrizione: Option<String>,
    pub modello: Option<String>,
    pub targa: Option<String>,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: Option<String>,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: Option<NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}
