use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;
use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

pub struct EditCourseCommand {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug)]
pub enum EditCourseError {
    CourseNotFound,
    DatabaseError(InfrastructureError),
}

pub struct EditCourseCommandHandler {
    repository: CourseRepository,
}

impl EditCourseCommandHandler {
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: EditCourseCommand) -> Result<bool, EditCourseError> {
        let mut course = self
            .repository
            .get_course_by_id(command.id)
            .await
            .map_err(|e| EditCourseError::DatabaseError(e.into()))?
            .ok_or(EditCourseError::CourseNotFound)?;

        course.update(
            command.title,
            command.slug,
            command.description,
            command.status,
        );

        let success = self
            .repository
            .edit_course(
                course.id,
                &course.title,
                &course.slug,
                course.description.as_deref(),
                &course.status,
            )
            .await
            .map_err(|e| EditCourseError::DatabaseError(e.into()))?;

        Ok(success)
    }
}
