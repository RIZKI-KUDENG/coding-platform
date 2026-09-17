use std::collections::HashMap;

use crate::modules::system::infrastructure::repositories::feature_flag_repository::FeatureFlagRepository;

pub struct GetFeatureFlagsQuery;

pub struct GetFeatureFlagsQueryHandler {
    repository: FeatureFlagRepository,
}

#[derive(Debug)]
pub enum GetFeatureFlagsError {
    DatabaseError(sqlx::Error),
}

impl GetFeatureFlagsQueryHandler {
    pub fn new(repository: FeatureFlagRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        _query: GetFeatureFlagsQuery,
    ) -> Result<HashMap<String, bool>, GetFeatureFlagsError> {
        self.repository
            .get_flags_map()
            .await
            .map_err(GetFeatureFlagsError::DatabaseError)
    }
}
