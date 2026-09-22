use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::learning::domain::entities::exercise_test_case::ExerciseTestCase;

#[derive(Clone)]
pub struct ExerciseTestCaseRepository {
    db: PgPool,
}

impl ExerciseTestCaseRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_test_case_by_exercise_id(
        &self,
        exercise_id: Uuid,
    ) -> Result<Vec<ExerciseTestCase>, sqlx::Error> {
        sqlx::query_as!(
            ExerciseTestCase,
            r#"
            SELECT
            id,
            exercise_id,
            input,
            expected_output,
            is_hidden,
            "order",
            created_at,
            updated_at
            FROM learning.m_exercise_test_cases
            WHERE exercise_id = $1
            ORDER BY "order" ASC
            "#,
            exercise_id
        )
        .fetch_all(&self.db)
        .await
    }
}
