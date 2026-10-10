use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;
use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

pub struct DeleteCourseCommand {
    pub course_id: Uuid,
}

#[derive(Debug)]
pub enum DeleteCourseError {
    CourseNotFound,
    DatabaseError(InfrastructureError),
}

pub struct DeleteCourseCommandHandler {
    repository: CourseRepository,
}

impl DeleteCourseCommandHandler {
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: DeleteCourseCommand) -> Result<(), DeleteCourseError> {
        let deleted = self
            .repository
            .delete_course(command.course_id)
            .await
            .map_err(|e| DeleteCourseError::DatabaseError(e.into()))?;

        if !deleted {
            return Err(DeleteCourseError::CourseNotFound);
        }
        Ok(())
    }
}
