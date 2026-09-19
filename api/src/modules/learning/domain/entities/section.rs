use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Clone)]
pub struct Section{
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
