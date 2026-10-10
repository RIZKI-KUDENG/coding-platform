use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct ExerciseTestCase {
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub input: String,
    pub expected_output: String,
    pub is_hidden: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
