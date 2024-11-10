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