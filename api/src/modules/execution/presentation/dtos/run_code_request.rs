use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RunCodeRequest {
    pub code: String,
    pub language: String,
}
