use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::identity::{
    application::security::session_token::hash_token,
    infrastructure::repositories::session_repository::SessionRepository,
};

pub struct ValidateSessionQuery {
    pub raw_token: String,
}

pub struct ValidateSessionQueryHandler {
    session_repository: SessionRepository,
}

#[derive(Debug)]
pub enum ValidateSessionError {
    InvalidOrExpiredToken,
    DatabaseError(InfrastructureError),
}

impl ValidateSessionQueryHandler {
    /// Public contract for other modules: build the handler without touching
    /// this module's infrastructure.
    pub fn from_pool(pool: sqlx::PgPool) -> Self {
        Self::new(SessionRepository::new(pool))
    }

    pub fn new(session_repository: SessionRepository) -> Self {
        Self { session_repository }
    }

    pub async fn handle(&self, query: ValidateSessionQuery) -> Result<Uuid, ValidateSessionError> {
        let token_hash = hash_token(&query.raw_token);

        let user_id = self
            .session_repository
            .find_user_by_valid_token(&token_hash)
            .await
            .map_err(|e| ValidateSessionError::DatabaseError(e.into()))?;

        user_id.ok_or(ValidateSessionError::InvalidOrExpiredToken)
    }
}
