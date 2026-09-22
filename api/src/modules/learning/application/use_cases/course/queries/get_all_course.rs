use crate::modules::learning::{
    domain::entities::course::Course,
    infrastructure::repositories::course_repository::CourseRepository,
};

pub struct GetAllCourseQuery;

#[derive(Debug)]
pub enum GetAllCourseError {
    DatabaseError(sqlx::Error),
}

pub struct GetAllCourseQueryHandler {
    repository: CourseRepository,
}

impl GetAllCourseQueryHandler {
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        _query: GetAllCourseQuery,
    ) -> Result<Vec<Course>, GetAllCourseError> {
        self.repository
            .get_all()
            .await
            .map_err(GetAllCourseError::DatabaseError)
    }
}
