use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;
use crate::modules::learning::domain::entities::course::Course;
use uuid::Uuid;

pub struct GetCourseByIdQuery {
    pub id: Uuid,
}
#[derive(Debug)]
pub enum GetCourseByIdError {
    CourseNotFound,
    DatabaseError(sqlx::Error)
}

pub struct GetCourseByIdQueryHandler {
    course_repository: CourseRepository,
}


impl GetCourseByIdQueryHandler {
    pub fn new(course_repository: CourseRepository) -> Self {
        Self { course_repository }
    }

    pub async fn handle(&self, query: GetCourseByIdQuery) -> Result<Course, GetCourseByIdError> {
        self.course_repository.get_course_by_id(query.id).await
            .map_err(|e| GetCourseByIdError::DatabaseError(e))
            .and_then(|course| {
                course.ok_or(GetCourseByIdError::CourseNotFound)
            })
    }
}
