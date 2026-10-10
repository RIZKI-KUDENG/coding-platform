use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    modules::{
        identity::{
            application::use_cases::auth::{
                commands::{
                    login::login_user_command::{LoginCommand, LoginCommandHandler},
                    register::register_user_command::{RegisterCommand, RegisterCommandHandler},
                },
                queries::get_current_user_query::{
                    GetCurrentUserQuery, GetCurrentUserQueryHandler,
                },
            },
            infrastructure::repositories::{
                session_repository::SessionRepository, user_repository::UserRepository,
            },
            presentation::dtos::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
        },
        shared::{authentication::authenticated_user::AuthenticatedUser, http::ApiError},
    },
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if request.email.trim().is_empty()
        || request.username.trim().is_empty()
        || request.password.trim().is_empty()
    {
        return Err(ApiError::validation(
            "Email, username, and password are required.",
        ));
    }

    let handler = RegisterCommandHandler::new(
        UserRepository::new(state.db.clone()),
        SessionRepository::new(state.db.clone()),
    );

    let result = handler
        .handle(RegisterCommand {
            username: request.username,
            email: request.email,
            password: request.password,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "data": AuthResponse::from(result) })),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if request.identifier.trim().is_empty() || request.password.trim().is_empty() {
        return Err(ApiError::validation(
            "email/username and password are required.",
        ));
    }

    let handler = LoginCommandHandler::new(
        UserRepository::new(state.db.clone()),
        SessionRepository::new(state.db.clone()),
    );

    let result = handler
        .handle(LoginCommand {
            identifier: request.identifier,
            password: request.password,
        })
        .await?;

    Ok(Json(json!({ "data": AuthResponse::from(result) })))
}

pub async fn get_me(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetCurrentUserQueryHandler::new(UserRepository::new(state.db.clone()));

    let current_user = handler
        .handle(GetCurrentUserQuery {
            user_id: user.user_id,
        })
        .await?;

    Ok(Json(json!({ "data": UserResponse::from(current_user) })))
}
