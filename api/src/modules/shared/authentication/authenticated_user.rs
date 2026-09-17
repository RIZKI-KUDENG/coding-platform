use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    modules::identity::{
        ValidateSessionQuery, ValidateSessionQueryHandler,
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

        let repo = SessionRepository::new(state.db.clone());
        let handler = ValidateSessionQueryHandler::new(repo);

        let query = ValidateSessionQuery {
            raw_token: token.to_string(),
        };

        match handler.handle(query).await {
            Ok(user_id) => Ok(AuthenticatedUser { user_id }),
            Err(_) => Err((
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
