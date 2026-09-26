use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;
use uuid::Uuid;


pub struct DeleteCourseCommand{
    pub course_id: Uuid,
}

#[derive(Debug)]
pub enum DeleteCourseError {
    CourseNotFound,
    DatabaseError(sqlx::Error),
}

pub struct DeleteCourseCommandHandler{
    repository: CourseRepository,
}

impl DeleteCourseCommandHandler{
    pub fn new(repository: CourseRepository) -> Self {
        Self{repository}
    }

    pub async fn handle(&self, command: DeleteCourseCommand) -> Result<(), DeleteCourseError> {
        let deleted = self.repository.delete_course(command.course_id)
            .await
            .map_err(DeleteCourseError::DatabaseError)?;

        if !deleted{
            return Err(DeleteCourseError::CourseNotFound);
        }
        Ok(())
    }
}
