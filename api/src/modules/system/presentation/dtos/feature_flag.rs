use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::system::application::use_cases::feature_flags::commands::edit_feature_flags_command::EditFeatureFlagsCommand;
use crate::modules::system::application::use_cases::feature_flags::queries::FeatureFlagStatus;
use crate::modules::system::domain::entities::feature_flag::FeatureFlag;

#[derive(Debug, Deserialize)]
pub struct EditFeatureFlagsRequest {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: i32,
    pub description: String,
}

impl From<EditFeatureFlagsRequest> for EditFeatureFlagsCommand {
    fn from(request: EditFeatureFlagsRequest) -> Self {
        Self {
            id: request.id,
            key: request.key,
            is_enabled: request.is_enabled,
            description: request.description,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FeatureFlagResponse {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: i32,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FeatureFlag> for FeatureFlagResponse {
    fn from(flag: FeatureFlag) -> Self {
        Self {
            id: flag.id,
            key: flag.key,
            is_enabled: flag.is_enabled,
            description: flag.description,
            parent_id: flag.parent_id,
            created_at: flag.created_at,
            updated_at: flag.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FeatureFlagStatusResponse {
    pub is_enabled: bool,
    pub is_sub_feature: bool,
    pub parent_id: Option<Uuid>,
}

impl From<FeatureFlagStatus> for FeatureFlagStatusResponse {
    fn from(status: FeatureFlagStatus) -> Self {
        Self {
            is_enabled: status.is_enabled,
            is_sub_feature: status.is_sub_feature,
            parent_id: status.parent_id,
        }
    }
}

pub fn status_map_response(
    statuses: HashMap<String, FeatureFlagStatus>,
) -> HashMap<String, FeatureFlagStatusResponse> {
    statuses.into_iter().map(|(k, v)| (k, v.into())).collect()
}
