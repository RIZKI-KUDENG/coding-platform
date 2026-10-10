use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    modules::{
        identity::{
            ValidateSessionError, ValidateSessionQuery, ValidateSessionQueryHandler,
            application::use_cases::auth::queries::get_current_user_query::{
                GetCurrentUserError, GetCurrentUserQuery, GetCurrentUserQueryHandler,
            },
            infrastructure::repositories::user_repository::UserRepository,
        },
        shared::{authentication::authenticated_user::bearer_token, http::ApiError},
    },
    state::AppState,
};

#[derive(Clone, Debug)]
pub struct AdminUser {
    pub user_id: Uuid,
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let token = bearer_token(req.headers()).ok_or_else(|| {
        ApiError::unauthorized("Missing or invalid Authorization header").into_response()
    })?;

    let user_id = ValidateSessionQueryHandler::from_pool(state.db.clone())
        .handle(ValidateSessionQuery {
            raw_token: token.to_string(),
        })
        .await
        .map_err(|err| match err {
            ValidateSessionError::InvalidOrExpiredToken => {
                ApiError::unauthorized("Invalid or expired token").into_response()
            }
            ValidateSessionError::DatabaseError(e) => ApiError::internal(&e).into_response(),
        })?;

    let user = GetCurrentUserQueryHandler::new(UserRepository::new(state.db.clone()))
        .handle(GetCurrentUserQuery { user_id })
        .await
        .map_err(|err| match err {
            GetCurrentUserError::UserNotFound => {
                ApiError::unauthorized("User not found").into_response()
            }
            GetCurrentUserError::DatabaseError(e) => ApiError::internal(&e).into_response(),
        })?;

    if user.role.as_deref() != Some("admin") {
        return Err(ApiError::forbidden("Access hanya untuk admin").into_response());
    }

    req.extensions_mut().insert(AdminUser { user_id });

    Ok(next.run(req).await)
}
