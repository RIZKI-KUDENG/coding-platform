use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    modules::identity::{
        application::use_cases::auth::commands::register::register_user_command::{
            RegisterCommand, RegisterCommandHandler, RegisterError,
        },
        infrastructure::repositories::{
            session_repository::SessionRepository, user_repository::UserRepository,
        },
        presentation::dtos::register_request::RegisterRequest,
    },
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    if request.email.trim().is_empty()
        || request.username.trim().is_empty()
        || request.password.trim().is_empty()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": {
                    "code": "VALIDATION_ERROR",
                    "message": "Email, username, and password are required."
                }
            })),
        );
    }

    let user_repo = UserRepository::new(state.db.clone());
    let session_repo = SessionRepository::new(state.db.clone());
    let handler = RegisterCommandHandler::new(user_repo, session_repo);

    let command = RegisterCommand {
        username: request.username,
        email: request.email,
        password: request.password,
    };

    match handler.handle(command).await {
        Ok(result) => (
            StatusCode::CREATED,
            Json(json!({
                "data": result
            })),
        ),
        Err(RegisterError::EmailAlreadyExists) => (
            StatusCode::CONFLICT,
            Json(json!({
                "error": {
                    "code": "EMAIL_ALREADY_EXISTS",
                    "message": "Email is already registered."
                }
            })),
        ),
        Err(RegisterError::UsernameAlreadyExists) => (
            StatusCode::CONFLICT,
            Json(json!({
                "error": {
                    "code": "USERNAME_ALREADY_EXISTS",
                    "message": "Username is already taken."
                }
            })),
        ),
        Err(RegisterError::PasswordHashing) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": {
                    "code": "INTERNAL_ERROR",
                    "message": "Failed to process password."
                }
            })),
        ),
        Err(RegisterError::Database(err)) => {
            eprintln!("Database error during registration: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": {
                        "code": "INTERNAL_ERROR",
                        "message": "An unexpected database error occurred."
                    }
                })),
            )
        }
    }
}
