use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub auth: bool,
    pub first_login: bool
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