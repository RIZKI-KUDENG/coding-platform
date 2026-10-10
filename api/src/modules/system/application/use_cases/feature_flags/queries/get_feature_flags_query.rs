use crate::modules::shared::error::InfrastructureError;
use std::collections::HashMap;

use uuid::Uuid;

use crate::modules::system::infrastructure::repositories::feature_flag_repository::FeatureFlagRepository;

pub struct GetFeatureFlagsQuery;

/// Effective status of a feature, taking its parent's state into account.
#[derive(Debug, Clone)]
pub struct FeatureFlagStatus {
    pub is_enabled: bool,
    pub is_sub_feature: bool,
    pub parent_id: Option<Uuid>,
}

pub struct GetFeatureFlagsQueryHandler {
    repository: FeatureFlagRepository,
}

#[derive(Debug)]
pub enum GetFeatureFlagsError {
    DatabaseError(InfrastructureError),
}

impl GetFeatureFlagsQueryHandler {
    pub fn new(repository: FeatureFlagRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        _query: GetFeatureFlagsQuery,
    ) -> Result<HashMap<String, FeatureFlagStatus>, GetFeatureFlagsError> {
        let flags = self
            .repository
            .get_all()
            .await
            .map_err(|e| GetFeatureFlagsError::DatabaseError(e.into()))?;

        let id_status: HashMap<Uuid, bool> = flags.iter().map(|f| (f.id, f.is_active())).collect();

        Ok(flags
            .iter()
            .map(|flag| {
                let parent_active = flag
                    .parent_id
                    .and_then(|pid| id_status.get(&pid).copied())
                    .unwrap_or(true);

                (
                    flag.key.clone(),
                    FeatureFlagStatus {
                        is_enabled: flag.is_active() && parent_active,
                        is_sub_feature: flag.is_sub_feature(),
                        parent_id: flag.parent_id,
                    },
                )
            })
            .collect())
    }
}
