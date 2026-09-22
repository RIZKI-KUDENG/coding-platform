use crate::modules::learning::domain::entities::course::Course;
use crate::modules::learning::infrastructure::repositories::course_repository::CourseRepository;

pub struct GetCourseAdminQueryHandler {
    repository: CourseRepository,
}

impl GetCourseAdminQueryHandler {
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(&self) -> Result<Vec<Course>, Box<dyn std::error::Error>> {
        let course = self.repository.get_all_admin().await?;

        if course.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No courses found",
            )));
        }

        Ok(course)
    }
}
