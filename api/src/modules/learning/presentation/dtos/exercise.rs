use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::modules::learning::domain::entities::exercise::Exercise;

#[derive(Debug, Serialize)]
pub struct ExerciseResponse {
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

impl From<Exercise> for ExerciseResponse {
    fn from(exercise: Exercise) -> Self {
        Self {
            id: exercise.id,
            lesson_id: exercise.lesson_id,
            slug: exercise.slug,
            title: exercise.title,
            description: exercise.description,
            starter_code: exercise.starter_code,
            language: exercise.language,
            order: exercise.order,
            xp_reward: exercise.xp_reward,
            status: exercise.status,
            created_at: exercise.created_at,
            updated_at: exercise.updated_at,
        }
    }
}
