use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FeatureFlag {
    pub fn is_active(&self) -> bool {
        self.is_enabled == 1
    }
}
