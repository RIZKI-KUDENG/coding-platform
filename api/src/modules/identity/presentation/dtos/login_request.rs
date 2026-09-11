use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}
