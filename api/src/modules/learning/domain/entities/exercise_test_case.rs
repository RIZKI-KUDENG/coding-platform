use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct ExerciseTestCase{
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub expected_output: String,
    pub is_hidden: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
