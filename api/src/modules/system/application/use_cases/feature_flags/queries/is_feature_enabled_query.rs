use crate::modules::system::infrastructure::repositories::feature_flag_repository::FeatureFlagRepository;

pub struct IsFeatureEnabledQuery {
    pub key: String,
    pub default_val: bool,
}

impl IsFeatureEnabledQuery {
    pub fn new(key: impl Into<String>, default_val: bool) -> Self {
        Self {
            key: key.into(),
            default_val,
        }
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
    ) -> Result<bool, IsFeatureEnabledError> {
        self.repository
            .is_enabled(&query.key, query.default_val)
            .await
            .map_err(IsFeatureEnabledError::DatabaseError)
    }
}
