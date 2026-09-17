use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmissionRequest {
    pub code: String,
    pub language: String,
}
