use crate::modules::system::infrastructure::repositories::feature_flag_repository::FeatureFlagRepository;

pub struct IsFeatureEnabledQuery {
    pub key: String,
}

impl IsFeatureEnabledQuery {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

pub struct IsFeatureEnabledQueryHandler {
    repository: FeatureFlagRepository,
}

#[derive(Debug)]
pub enum IsFeatureEnabledError {
    DatabaseError(sqlx::Error),
}

impl IsFeatureEnabledQueryHandler {
    pub fn new(repository: FeatureFlagRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: IsFeatureEnabledQuery,
    ) -> Result<Option<bool>, IsFeatureEnabledError> {
        self.repository
            .is_enabled(&query.key)
            .await
            .map_err(IsFeatureEnabledError::DatabaseError)
    }
}
