use crate::modules::learning::infrastructure::repositories::lesson_repository::LessonRepository;
use crate::modules::learning::domain::entities::lesson::Lesson;

use uuid::Uuid;


pub struct GetLessonByCourseIdQuery{
    pub id: Uuid,
}

#[derive(Debug)]
pub enum GetLessonByCourseIdError {
    DatabaseError(sqlx::Error),
}

pub struct GetLessonByCourseIdHandler {
    repository: LessonRepository,
}

impl GetLessonByCourseIdHandler{
    pub fn new(repository: LessonRepository) -> Self{
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetLessonByCourseIdQuery,
    ) -> Result<Vec<Lesson>, GetLessonByCourseIdError> {
        self.repository
            .get_by_course_id(query.id)
            .await
            .map_err(GetLessonByCourseIdError::DatabaseError)
    }
}
