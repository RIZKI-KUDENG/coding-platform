use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;

pub struct CreateCourseCommand {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub status: String,
}

pub struct CreateCourseCommandHandler {
    repository: CourseRepository,
}

impl CreateCourseCommandHandler {
    pub async fn handle(&self, command: CreateCourseCommand) -> Result<(), sqlx::Error> {
        self.repository
            .create_course(
                &command.title,
                &command.slug,
                &command.description,
                &command.status,
            )
            .await?;

        Ok(())
    }
}
