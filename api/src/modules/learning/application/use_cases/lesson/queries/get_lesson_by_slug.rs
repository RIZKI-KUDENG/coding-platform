use crate::modules::learning::infrastructure::repositories::lesson_repository::LessonRepository;
use crate::modules::learning::domain::entities::lesson::Lesson;


pub struct GetLessonBySlugQuery{
    pub slug: String,
}

#[derive(Debug)]
pub enum GetLessonBySlugError {
    NotFound,
    DatabaseError(sqlx::Error),
}

pub struct GetLessonBySlugQueryHandler{
    repository: LessonRepository,
}

impl GetLessonBySlugQueryHandler{
    pub fn new(repository: LessonRepository) -> Self {
        Self{repository}
    }

    pub async fn handle(&self, query: GetLessonBySlugQuery) -> Result<Lesson, GetLessonBySlugError> {
        self.repository.get_by_slug(&query.slug)
            .await
            .map_err(|e| GetLessonBySlugError::DatabaseError(e))
            .and_then(|o| o.ok_or(GetLessonBySlugError::NotFound))
    }
}
