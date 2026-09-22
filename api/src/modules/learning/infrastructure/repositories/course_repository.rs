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
            WHERE status = 'PUBLISHED'
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

    pub async fn get_all_admin(&self) -> Result<Vec<Course>, sqlx::Error> {
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
    pub async fn edit_course(
        &self,
        uuid: Uuid,
        title: &str,
        slug: &str,
        description: Option<&str>,
        status: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE learning.m_courses
            SET title = $1, slug = $2, description = $3, status = $4,
                updated_at = NOW()
            WHERE id = $5
            "#,
            title,
            slug,
            description,
            status,
            uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
    pub async fn create_course(
        &self,
        title: &str,
        slug: &str,
        description: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO learning.m_courses (title, slug, description, status)
            VALUES ($1, $2, $3, $4)
            "#,
            title,
            slug,
            description,
            status
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
    pub async fn delete_course(&self, uuid: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM learning.m_courses
            WHERE id = $1
            "#,
            uuid
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
}
