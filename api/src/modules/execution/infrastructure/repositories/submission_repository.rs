use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::execution::domain::entities::submission::{Submission, SubmissionStatus};

#[derive(Clone)]
pub struct SubmissionRepository {
    pool: PgPool,
}

impl SubmissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_submission(
        &self,
        user_id: Uuid,
        exercise_id: Uuid,
        code: &str,
        language: &str,
    ) -> Result<Submission, sqlx::Error> {
        sqlx::query_as!(
            Submission,
            r#"
            INSERT INTO execution.t_submissions(
            user_id,
            exercise_id,
            code,
            language,
            status
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
            id,
            user_id,
            exercise_id,
            code,
            language,
            status,
            execution_time_ms,
            created_at,
            updated_at
            "#,
            user_id,
            exercise_id,
            code,
            language,
            "PENDING"
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        status: SubmissionStatus,
        execution_time_ms: Option<i32>,
    ) -> Result<Submission, sqlx::Error> {
        sqlx::query_as!(
            Submission,
            r#"
            UPDATE execution.t_submissions
            SET
            status = $2,
            execution_time_ms = $3,
            updated_at = NOW()
            WHERE id = $1
            RETURNING
            id,
            user_id,
            exercise_id,
            code,
            language,
            status,
            execution_time_ms,
            created_at,
            updated_at
            "#,
            id,
            status.to_string(),
            execution_time_ms,
        )
        .fetch_one(&self.pool)
        .await
    }
}
