use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::identity::application::security::password_hasher::verify_password;
use crate::modules::identity::application::security::session_token::generate_session_token;
use crate::modules::identity::infrastructure::repositories::session_repository::SessionRepository;
use crate::modules::identity::infrastructure::repositories::user_repository::UserRepository;

pub struct LoginCommand {
    pub identifier: String,
    pub password: String,
}

pub struct LoginCommandHandler {
    user_repository: UserRepository,
    session_repository: SessionRepository,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    DatabaseError(sqlx::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub user: UserSummary,
    pub access_token: String,
}

impl LoginCommandHandler {
    pub fn new(user_repository: UserRepository, session_repository: SessionRepository) -> Self {
        Self {
            user_repository,
            session_repository,
        }
    }

    pub async fn handle(&self, command: LoginCommand) -> Result<LoginResult, LoginError> {
        let user = self
            .user_repository
            .find_by_email_or_username(&command.identifier)
            .await
            .map_err(LoginError::DatabaseError)?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(LoginError::InvalidCredentials);
            }
        };

        let valid = verify_password(&command.password, &user.password)
            .map_err(|_| LoginError::InvalidCredentials)?;

        if !valid {
            return Err(LoginError::InvalidCredentials);
        }

        let session_token = generate_session_token();
        let expires_at = Utc::now() + Duration::days(7);

        self.session_repository
            .create(user.id, session_token.hash, expires_at)
            .await
            .map_err(LoginError::DatabaseError)?;

        Ok(LoginResult {
            user: UserSummary {
                id: user.id,
                email: user.email,
                username: user.username,
            },
            access_token: session_token.raw,
        })
    }
}
