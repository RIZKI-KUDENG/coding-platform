//! Maps identity application errors to the public API error envelope.

use axum::http::StatusCode;

use crate::modules::identity::application::use_cases::auth::{
    commands::{
        login::login_user_command::LoginError, register::register_user_command::RegisterError,
    },
    queries::get_current_user_query::GetCurrentUserError,
};
use crate::modules::shared::http::ApiError;

impl From<RegisterError> for ApiError {
    fn from(err: RegisterError) -> Self {
        match err {
            RegisterError::EmailAlreadyExists => ApiError::new(
                StatusCode::CONFLICT,
                "EMAIL_ALREADY_EXISTS",
                "Email is already registered.",
            ),
            RegisterError::UsernameAlreadyExists => ApiError::new(
                StatusCode::CONFLICT,
                "USERNAME_ALREADY_EXISTS",
                "Username is already taken.",
            ),
            RegisterError::PasswordHashing => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Failed to process password.",
            ),
            RegisterError::Database(e) => ApiError::internal(&e),
        }
    }
}

impl From<LoginError> for ApiError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::InvalidCredentials => ApiError::new(
                StatusCode::UNAUTHORIZED,
                "AUTH_INVALID_CREDENTIALS",
                "Invalid email/username or password.",
            ),
            LoginError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetCurrentUserError> for ApiError {
    fn from(err: GetCurrentUserError) -> Self {
        match err {
            GetCurrentUserError::UserNotFound => {
                ApiError::not_found("USER_NOT_FOUND", "User not found.")
            }
            GetCurrentUserError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}
