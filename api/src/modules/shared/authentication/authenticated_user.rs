use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    modules::{
        identity::{ValidateSessionError, ValidateSessionQuery, ValidateSessionQueryHandler},
        shared::http::ApiError,
    },
    state::AppState,
};

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

/// Extracts a non-empty bearer token from the `Authorization` header.
pub fn bearer_token(headers: &axum::http::HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = bearer_token(&parts.headers).ok_or_else(|| {
            ApiError::unauthorized("Missing or invalid Authorization header").into_response()
        })?;

        let handler = ValidateSessionQueryHandler::from_pool(state.db.clone());

        match handler
            .handle(ValidateSessionQuery {
                raw_token: token.to_string(),
            })
            .await
        {
            Ok(user_id) => Ok(AuthenticatedUser { user_id }),
            Err(ValidateSessionError::InvalidOrExpiredToken) => {
                Err(ApiError::unauthorized("Invalid or expired session token").into_response())
            }
            Err(ValidateSessionError::DatabaseError(e)) => {
                Err(ApiError::internal(&e).into_response())
            }
        }
    }
}
