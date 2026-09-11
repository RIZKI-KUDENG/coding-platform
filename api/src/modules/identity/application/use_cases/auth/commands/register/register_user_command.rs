
use crate::modules::identity::domain::entities::user::User;
use crate::modules::identity::infrastructure::repositories::user_repository::UserRepository;
use crate::modules::identity::application::security::password_hasher::hash_password;

pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterCommandHandler {
    user_repository: UserRepository,
}

#[derive(Debug)]
pub enum RegisterError{
    PasswordHashing,
    Database(sqlx::Error),
}

impl RegisterCommandHandler {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: RegisterCommand) -> Result<User, RegisterError> {
           let hashed_password = hash_password(&command.password)
            .map_err(|_| RegisterError::PasswordHashing)?;

           self.user_repository.create_user(&command.email, &command.username, &hashed_password)
               .await
               .map_err(RegisterError::Database)
    }
}
