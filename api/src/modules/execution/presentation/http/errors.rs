//! Maps execution application errors to the public API error envelope.

use axum::http::StatusCode;

use crate::modules::execution::application::errors::ExecutionError;
use crate::modules::shared::http::ApiError;

impl From<ExecutionError> for ApiError {
    fn from(err: ExecutionError) -> Self {
        match err {
            ExecutionError::RunnerUnavailable(e) => {
                eprintln!("Runner failure: {}", e.detail());
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "EXECUTION_FAILED",
                    "Code execution is currently unavailable.",
                )
            }
            ExecutionError::Persistence(e) => ApiError::internal(&e),
        }
    }
}
