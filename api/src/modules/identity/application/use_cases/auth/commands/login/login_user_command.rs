use crate::modules::identity::infrastructure::repositories::user_repository::UserRepository;
use crate::modules::identity::application::security::password_hasher::verify_password;

pub struct LoginCommand {
    pub identifier: String,
    pub password: String,
}

pub struct LoginCommandHandler {
    user_repository: UserRepository,
}

pub enum LoginError {
    InvalidCredentials,
    DatabaseError(sqlx::Error),
}

pub struct LoginResult {
    session_id: String,
}

impl LoginCommandHandler {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
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

        if !valid{
            return Err(LoginError::InvalidCredentials);
        }

        Ok(LoginResult { session_id:  })
    }
}
