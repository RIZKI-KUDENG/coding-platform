use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Exercise {
    pub id: Uuid,
    pub lesson_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub starter_code: Option<String>,
    pub language: String,
    pub order: i32,
    pub xp_reward: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
