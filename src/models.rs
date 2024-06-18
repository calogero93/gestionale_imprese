use axum_sessions::async_session::chrono;
use diesel::prelude::*;
use serde::Serialize;
use super::schema::*;


#[derive(Queryable, Serialize, AsChangeset)]
pub struct Autovetture {
    pub id: i32,
    pub descrizione: Option<String>,
    pub modello: String,
    pub targa: String,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: String,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "autovettures"]
pub struct NewAutovettura {
    pub descrizione: Option<String>,
    pub modello: String,
    pub targa: String,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: String,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Queryable, QueryableByName, Serialize, AsChangeset)]
pub struct Dipendenti {
    pub id: i32,
    pub nome: String,
    pub cognome: String,
    pub matricola: Option<String>,
    pub data_di_nascita: chrono::NaiveDate,
    pub luogo_di_nascita: String,
    pub codice_fiscale: String,
    pub impresa_id: Option<i32>,
    pub qualifica: Option<i32>,
    pub mansione: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "dipendentis"]
pub struct NewDipendente {
    pub nome: String,
    pub cognome: String,
    pub matricola: Option<String>,
    pub data_di_nascita: chrono::NaiveDate,
    pub luogo_di_nascita: String,
    pub codice_fiscale: String,
    pub impresa_id: Option<i32>,
    pub qualifica: Option<i32>,
    pub mansione: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Imprese {
    pub id: i32,
    pub ragion_sociale: Option<String>,
    pub indirizzo: String,
    pub targa: String,
    pub partita_iva: String,
    pub proprieta: String,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "impreses"]
pub struct NewImpresa {
    pub ragion_sociale: Option<String>,
    pub indirizzo: String,
    pub targa: String,
    pub partita_iva: String,
    pub proprieta: String,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct ImpreseAssociateUtenti {
    pub id: i32,
    pub utente_id: Option<i32>,
    pub impresa_id: Option<i32>,
}

#[derive(Insertable, Serialize)]
#[table_name = "imprese_associate_utentis"]
pub struct NewImpreseAssociateUtente {
    pub utente_id: Option<i32>,
    pub impresa_id: Option<i32>,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct ImpreseCollegate {
    pub id: i32,
    pub impresa_id: Option<i32>,
    pub ruolo_impresa: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "imprese_collegates"]
pub struct NewImpreseCollegata {
    pub impresa_id: Option<i32>,
    pub ruolo_impresa: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Mansioni {
    pub id: i32,
    pub descrizione: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "mansionis"]
pub struct NewMansione {
    pub descrizione: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Mezzi {
    pub id: i32,
    pub descrizione: Option<String>,
    pub modello: String,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: String,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "mezzis"]
pub struct NewMezzo {
    pub descrizione: Option<String>,
    pub modello: String,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: String,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Opere {
    pub id: i32,
    pub descrizione: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "operes"]
pub struct NewOpera {
    pub descrizione: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Qualifiche {
    pub id: i32,
    pub descrizione: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "qualifiches"]
pub struct NewQualifica {
    pub descrizione: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct TipiProprieta {
    pub id: i32,
    pub descrizione: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "tipi_proprietas"]
pub struct NewTipoProprieta {
    pub descrizione: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Utenti {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub nome: String,
    pub cognome: String,
    pub impresa_id: Option<i32>,
    pub utente: String,
    pub autorizazzione: bool,
}

#[derive(Insertable, Serialize)]
#[table_name = "utentis"]
pub struct NewUtente {
    pub username: String,
    pub password: String,
    pub nome: String,
    pub cognome: String,
    pub impresa_id: Option<i32>,
    pub utente: String,
    pub autorizazzione: bool,
}
