use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug,Deserialize)]
pub struct SubmissionRequest {
    pub code: String,
}
