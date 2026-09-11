use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::identity::application::security::password_hasher::hash_password;
use crate::modules::identity::application::security::session_token::generate_session_token;
use crate::modules::identity::infrastructure::repositories::session_repository::SessionRepository;
use crate::modules::identity::infrastructure::repositories::user_repository::UserRepository;

pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterCommandHandler {
    user_repository: UserRepository,
    session_repository: SessionRepository,
}

#[derive(Debug)]
pub enum RegisterError {
    EmailAlreadyExists,
    UsernameAlreadyExists,
    PasswordHashing,
    Database(sqlx::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResult {
    pub user: UserSummary,
    pub access_token: String,
}

impl RegisterCommandHandler {
    pub fn new(user_repository: UserRepository, session_repository: SessionRepository) -> Self {
        Self {
            user_repository,
            session_repository,
        }
    }

    pub async fn handle(&self, command: RegisterCommand) -> Result<RegisterResult, RegisterError> {
        let hashed_password =
            hash_password(&command.password).map_err(|_| RegisterError::PasswordHashing)?;

        let user = match self
            .user_repository
            .create_user(&command.email, &command.username, &hashed_password)
            .await
        {
            Ok(user) => user,
            Err(sqlx::Error::Database(err)) => {
                if let Some(constraint) = err.constraint() {
                    if constraint.contains("email") {
                        return Err(RegisterError::EmailAlreadyExists);
                    }
                    if constraint.contains("username") {
                        return Err(RegisterError::UsernameAlreadyExists);
                    }
                }
                return Err(RegisterError::Database(sqlx::Error::Database(err)));
            }
            Err(err) => return Err(RegisterError::Database(err)),
        };

        let session_token = generate_session_token();
        let expires_at = Utc::now() + Duration::days(7);

        self.session_repository
            .create(user.id, session_token.hash, expires_at)
            .await
            .map_err(RegisterError::Database)?;

        Ok(RegisterResult {
            user: UserSummary {
                id: user.id,
                email: user.email,
                username: user.username,
            },
            access_token: session_token.raw,
        })
    }
}
