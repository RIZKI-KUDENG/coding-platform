use sqlx::PgPool;
use uuid::Uuid;


use crate::modules::learning::domain::entities::exercise::Exercise;


pub struct ExerciseRepository {
    db: PgPool,
}

impl ExerciseRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_exercise_by_lesson_id(&self, lesson_id: Uuid) -> Result<Vec<Exercise>, sqlx::Error> {
        sqlx::query_as!(
            Exercise,
            r#"
            SELECT
            e.id,
            e.lesson_id,
            e.slug,
            e.title,
            e.status,
            e.description,
            e.starter_code,
            e.language,
            e."order",
            e.xp_reward,
            e.created_at,
            e.updated_at
            FROM learning.m_exercises e
            WHERE e.lesson_id = $1
            ORDER BY e."order" ASC
            "#,
            lesson_id,
        ).fetch_all(&self.db).await
    }

    pub async fn get_exercise_by_id(&self, id:Uuid) -> Result<Option<Exercise>, sqlx::Error> {
        sqlx::query_as!(
            Exercise,
            r#"
            SELECT
            id,
            lesson_id,
            slug,
            title,
            status,
            description,
            starter_code,
            language,
            "order",
            xp_reward,
            created_at,
            updated_at
            FROM learning.m_exercises
            WHERE id = $1
            "#,
            id
        ).fetch_optional(&self.db)
        .await
    }
}
