use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
