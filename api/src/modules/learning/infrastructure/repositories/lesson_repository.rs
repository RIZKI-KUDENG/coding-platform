use sqlx::PgPool;
use uuid::Uuid;


use crate::modules::learning::domain::entities::lesson::Lesson;

pub struct LessonRepository {
    db: PgPool,
}

impl LessonRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_by_course_id(&self, course_id: Uuid) -> Result<Vec<Lesson>, sqlx::Error> {
       sqlx::query_as!(
           Lesson,
           r#"
           SELECT
           l.id,
           l.title,
           l.slug,
           l.description,
           l.status,
           l.created_at,
           l.updated_at
           FROM learning.m_sections s
           JOIN learning.m_section_lessons sl
           ON sl.section_id = s.id
           JOIN learning.m_lessons l ON l.id = sl.lesson_id
           WHERE s.course_id = $1
           ORDER BY s."order" ASC, sl."order" ASC
           "#,
           course_id,
       ).fetch_all(&self.db)
    .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Lesson>, sqlx::Error>{
        sqlx::query_as!(
            Lesson,
            r#"
            SELECT
            id,
            title,
            slug,
            description,
            status,
            created_at,
            updated_at
            FROM learning.m_lessons
            WHERE id = $1
            "#,
            id
        ).fetch_optional(&self.db)
        .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Lesson>, sqlx::Error>{
        sqlx::query_as!(
            Lesson,
            r#"
            SELECT
            id,
            title,
            slug,
            description,
            status,
            created_at,
            updated_at
            FROM learning.m_lessons
            WHERE slug = $1
            "#,
            slug
        ).fetch_optional(&self.db)
        .await
    }
}
