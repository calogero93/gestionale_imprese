use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}
