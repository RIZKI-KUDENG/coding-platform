use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::modules::learning::domain::entities::lesson::Lesson;

#[derive(Debug, Serialize)]
pub struct LessonResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Lesson> for LessonResponse {
    fn from(lesson: Lesson) -> Self {
        Self {
            id: lesson.id,
            title: lesson.title,
            slug: lesson.slug,
            description: lesson.description,
            status: lesson.status,
            created_at: lesson.created_at,
            updated_at: lesson.updated_at,
        }
    }
}
