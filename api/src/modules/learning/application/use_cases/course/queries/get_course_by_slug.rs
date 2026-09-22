use crate::modules::learning::domain::entities::course::Course;
use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;

pub struct GetCourseBySlugQuery {
    pub slug: String,
}

#[derive(Debug)]
pub enum GetCourseBySlugError {
    CourseNotFound,
    DatabaseError(sqlx::Error),
}

pub struct GetCourseBySlugQueryHandler {
    course_repository: CourseRepository,
}

impl GetCourseBySlugQueryHandler {
    pub fn new(course_repository: CourseRepository) -> Self {
        Self { course_repository }
    }

    pub async fn handle(
        &self,
        query: GetCourseBySlugQuery,
    ) -> Result<Course, GetCourseBySlugError> {
        let course = self
            .course_repository
            .get_by_slug(&query.slug)
            .await
            .map_err(GetCourseBySlugError::DatabaseError)?;
        course.ok_or(GetCourseBySlugError::CourseNotFound)
    }
}
