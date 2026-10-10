use serde::Serialize;
use uuid::Uuid;

use crate::modules::identity::application::use_cases::auth::commands::login::login_user_command::LoginResult;
use crate::modules::identity::application::use_cases::auth::commands::register::register_user_command::RegisterResult;
use crate::modules::identity::application::use_cases::auth::queries::get_current_user_query::CurrentUser;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
}

impl From<CurrentUser> for UserResponse {
    fn from(user: CurrentUser) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
        }
    }
}

impl From<LoginResult> for AuthResponse {
    fn from(result: LoginResult) -> Self {
        Self {
            user: UserResponse {
                id: result.user.id,
                email: result.user.email,
                username: result.user.username,
                role: result.user.role,
            },
            access_token: result.access_token,
        }
    }
}

impl From<RegisterResult> for AuthResponse {
    fn from(result: RegisterResult) -> Self {
        Self {
            user: UserResponse {
                id: result.user.id,
                email: result.user.email,
                username: result.user.username,
                role: result.user.role,
            },
            access_token: result.access_token,
        }
    }
}
