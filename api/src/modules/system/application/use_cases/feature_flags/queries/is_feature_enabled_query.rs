use crate::modules::shared::error::InfrastructureError;
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
    DatabaseError(InfrastructureError),
}

impl IsFeatureEnabledQueryHandler {
    pub fn new(repository: FeatureFlagRepository) -> Self {
        Self { repository }
    }

    /// Public contract for other modules: build the handler without touching
    /// this module's infrastructure.
    pub fn from_pool(pool: sqlx::PgPool) -> Self {
        Self::new(FeatureFlagRepository::new(pool))
    }

    pub async fn handle(
        &self,
        query: IsFeatureEnabledQuery,
    ) -> Result<Option<bool>, IsFeatureEnabledError> {
        self.repository
            .is_enabled(&query.key)
            .await
            .map_err(|e| IsFeatureEnabledError::DatabaseError(e.into()))
    }
}
