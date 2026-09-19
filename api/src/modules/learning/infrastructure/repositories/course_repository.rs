use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::learning::domain::entities::course::Course;

#[derive(Clone, Debug)]
pub struct CourseRepository {
    pool: PgPool,
}

impl CourseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn get_all(&self) -> Result<Vec<Course>, sqlx::Error> {
        sqlx::query_as!(
            Course,
            r#"
            SELECT
            id,
            title,
            slug,
            description,
            status,
            created_at,
            updated_at
            FROM learning.m_courses
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Course>, sqlx::Error> {
        sqlx::query_as!(
            Course,
            r#"
            SELECT
            id,
            title,
            slug,
            description,
            status,
            created_at,
            updated_at
            FROM learning.m_courses
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
    }
    pub async fn get_course_by_id(&self, uuid: Uuid) -> Result<Option<Course>, sqlx::Error> {
        sqlx::query_as!(
            Course,
            r#"
            SELECT
            id,
            title,
            slug,
            description,
            status,
            created_at,
            updated_at
            FROM learning.m_courses
            WHERE id = $1
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await
    }
}
