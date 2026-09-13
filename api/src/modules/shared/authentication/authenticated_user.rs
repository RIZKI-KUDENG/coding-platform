use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    modules::identity::{
        application::security::session_token::hash_token,
        infrastructure::repositories::session_repository::SessionRepository,
    },
    state::AppState,
};

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
        {
            Some(token) if !token.is_empty() => token,
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": {
                            "code": "AUTH_UNAUTHORIZED",
                            "message": "Missing or invalid Authorization header"
                        }
                    })),
                )
                    .into_response());
            }
        };

        let token_hash = hash_token(token);

        let repo = SessionRepository::new(state.db.clone());

        match repo.find_user_by_valid_token(&token_hash).await {
            Ok(Some(user_id)) => Ok(AuthenticatedUser { user_id }),

            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "code": "AUTH_UNAUTHORIZED",
                        "message": "Invalid or expired session token"
                    }
                })),
            )
                .into_response()),
        }
    }
}
