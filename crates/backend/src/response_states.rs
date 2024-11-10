use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub auth: Option<bool>,
    pub first_login: Option<bool>
}


#[derive(Serialize)]
pub struct UtentiResponse {
    pub id: i32,
    pub impresa: String,
    pub nome: String,
    pub cognome: String,
    pub username: String,
    pub revocato: bool
}


#[derive(Serialize)]
pub struct SettimanaleResponse {
    pub data_settimanale: String,
    pub utente_id: i32,
    pub luogo_di_nascita: String,
    pub data_di_nascita: DateTime<FixedOffset>,
    pub tipo_proprieta: i32,
    pub proprieta: String,
    pub impresa_id: i32,
    pub opera_id: i32,
    pub mezzo_id: Option<i32>,
    pub autovettura_id: i32,
    pub matricola: Option<String>,
    pub targa: String,
    pub nome: String,
    pub cognome: String
}