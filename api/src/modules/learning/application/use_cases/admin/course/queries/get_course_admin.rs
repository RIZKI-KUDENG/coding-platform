use crate::modules::learning::domain::entities::course::Course;
use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;
use crate::modules::shared::error::InfrastructureError;

pub struct GetCourseAdminQueryHandler {
    repository: CourseRepository,
}

impl GetCourseAdminQueryHandler {
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    /// Returns every course for admin; an empty list is a valid result.
    pub async fn handle(&self) -> Result<Vec<Course>, InfrastructureError> {
        Ok(self.repository.get_all_admin().await?)
    }
}
