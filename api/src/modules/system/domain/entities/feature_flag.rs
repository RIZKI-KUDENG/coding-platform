use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: i32,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FeatureFlag {
    pub fn is_active(&self) -> bool {
        self.is_enabled == 1
    }

    pub fn is_sub_feature(&self) -> bool {
        self.parent_id.is_some()
    }

    pub fn is_major_feature(&self) -> bool {
        self.parent_id.is_none()
    }
}
