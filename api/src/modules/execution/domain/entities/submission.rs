use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;


#[derive(Clone,Debug)]
pub struct Submission{
    pub id: Uuid,
    pub user_id: Uuid,
    pub exercise_id: Uuid,
    pub code: String,
    pub language: String,
    pub status: String,
    pub execution_time_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone,Debug, Serialize,Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubmissionStatus{
    Pending,
    Running,
    Passed,
    Failed,
    Timeout,
    CompileError,
    RuntimeError
}

impl fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            SubmissionStatus::Pending => "PENDING",
            SubmissionStatus::Running => "RUNNING",
            SubmissionStatus::Passed => "PASSED",
            SubmissionStatus::Failed => "FAILED",
            SubmissionStatus::Timeout => "TIMEOUT",
            SubmissionStatus::CompileError => "COMPILE_ERROR",
            SubmissionStatus::RuntimeError => "RUNTIME_ERROR",
        };

        write!(f, "{value}")
    }
}
