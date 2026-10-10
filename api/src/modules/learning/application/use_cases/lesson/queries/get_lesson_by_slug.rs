use crate::modules::learning::domain::entities::lesson::Lesson;
use crate::modules::learning::infrastructure::repositories::lesson_repository::LessonRepository;
use crate::modules::shared::error::InfrastructureError;

pub struct GetLessonBySlugQuery {
    pub slug: String,
}

#[derive(Debug)]
pub enum GetLessonBySlugError {
    NotFound,
    DatabaseError(InfrastructureError),
}

pub struct GetLessonBySlugQueryHandler {
    repository: LessonRepository,
}

impl GetLessonBySlugQueryHandler {
    pub fn new(repository: LessonRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetLessonBySlugQuery,
    ) -> Result<Lesson, GetLessonBySlugError> {
        self.repository
            .get_by_slug(&query.slug)
            .await
            .map_err(|e| GetLessonBySlugError::DatabaseError(e.into()))
            .and_then(|o| o.ok_or(GetLessonBySlugError::NotFound))
    }
}
