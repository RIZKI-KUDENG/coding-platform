use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::system::domain::entities::feature_flag::FeatureFlag;
use crate::modules::system::infrastructure::repositories::feature_flag_repository::FeatureFlagRepository;

pub struct EditFeatureFlagsCommand {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: i32,
    pub description: String,
}

pub struct EditFeatureFlagsCommandHandler {
    pub repository: FeatureFlagRepository,
}

impl EditFeatureFlagsCommandHandler {
    pub fn new(repository: FeatureFlagRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: EditFeatureFlagsCommand,
    ) -> Result<Option<FeatureFlag>, InfrastructureError> {
        let result = self
            .repository
            .edit_feature(
                command.id,
                &command.key,
                command.is_enabled,
                Some(&command.description),
            )
            .await?;
        Ok(result)
    }
}
