use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::identity::infrastructure::repositories::user_repository::UserRepository;

pub struct GetCurrentUserQuery {
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub role: Option<String>,
}

#[derive(Debug)]
pub enum GetCurrentUserError {
    UserNotFound,
    DatabaseError(InfrastructureError),
}

pub struct GetCurrentUserQueryHandler {
    user_repository: UserRepository,
}

impl GetCurrentUserQueryHandler {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn handle(
        &self,
        query: GetCurrentUserQuery,
    ) -> Result<CurrentUser, GetCurrentUserError> {
        let user = self
            .user_repository
            .find_by_id(query.user_id)
            .await
            .map_err(|e| GetCurrentUserError::DatabaseError(e.into()))?
            .ok_or(GetCurrentUserError::UserNotFound)?;

        Ok(CurrentUser {
            id: user.id,
            email: user.email,
            username: user.username,
            role: user.role,
        })
    }
}
