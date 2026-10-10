use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::modules::shared::error::InfrastructureError;

/// Error envelope defined in `docs/API-CONTRACT.md` (section 26):
/// `{ "error": { "code": "...", "message": "..." } }`.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
    feature: Option<String>,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            feature: None,
        }
    }

    /// Attaches the feature key; clients use it to render maintenance states.
    pub fn with_feature(mut self, key: impl Into<String>) -> Self {
        self.feature = Some(key.into());
        self
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "AUTH_UNAUTHORIZED", message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "FORBIDDEN_RESOURCE", message)
    }

    pub fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, code, message)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
    }

    /// Logs the infrastructure detail and returns a generic 500 to the client.
    pub fn internal(err: &InfrastructureError) -> Self {
        eprintln!("Internal error: {}", err.detail());
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            "An unexpected error occurred.",
        )
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut error = json!({
            "code": self.code,
            "message": self.message,
        });
        if let Some(feature) = self.feature {
            error["feature"] = json!(feature);
        }

        (self.status, Json(json!({ "error": error }))).into_response()
    }
}
