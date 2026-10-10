use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::learning::domain::entities::lesson::Lesson;
use crate::modules::learning::infrastructure::repositories::lesson_repository::LessonRepository;

pub struct GetLessonByIdQuery {
    pub id: Uuid,
}

#[derive(Debug)]
pub enum GetLessonByIdError {
    NotFound,
    DatabaseError(InfrastructureError),
}

pub struct GetLessonByIdQueryHandler {
    repository: LessonRepository,
}

impl GetLessonByIdQueryHandler {
    pub fn new(repository: LessonRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetLessonByIdQuery) -> Result<Lesson, GetLessonByIdError> {
        self.repository
            .get_by_id(query.id)
            .await
            .map_err(|e| GetLessonByIdError::DatabaseError(e.into()))
            .and_then(|lesson| lesson.ok_or(GetLessonByIdError::NotFound))
    }
}
