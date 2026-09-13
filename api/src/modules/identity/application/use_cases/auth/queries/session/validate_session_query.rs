use uuid::Uuid;

use crate::modules::identity::{
    application::security::session_token::hash_token,
    infrastructure::repositories::session_repository::SessionRepository
};

pub struct ValidateSessionQuery{
    pub raw_token: String
}

pub struct ValidateSessionQueryHandler{
    session_repository: SessionRepository,
}

#[derive(Debug)]
pub enum ValidateSessionError{
    InvalidOrExpiredToken,
    DatabaseError(sqlx::Error)
}

impl ValidateSessionQueryHandler{
    pub fn new(session_repository: SessionRepository) -> Self{
        Self {session_repository}
    }

    pub async fn handle(
        &self,
        query: ValidateSessionQuery
    ) -> Result<Uuid, ValidateSessionError> {
       let  token_hash = hash_token(&query.raw_token);

        let user_id = self.
            session_repository
            .find_user_by_valid_token(&token_hash)
            .await
            .map_err(ValidateSessionError::DatabaseError)?;


        user_id.ok_or(ValidateSessionError::InvalidOrExpiredToken)
    }

}
