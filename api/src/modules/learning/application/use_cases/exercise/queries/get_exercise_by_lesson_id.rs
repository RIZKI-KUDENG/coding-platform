use crate::modules::learning::domain::entities::exercise::Exercise;
use crate::modules::learning::infrastructure::repositories::exercise_repository::ExerciseRepository;

use uuid::Uuid;

pub struct GetExerciseByLessonIdQuery {
    pub id: Uuid,
}

#[derive(Debug)]
pub enum GetExerciseByLessonIdError {
    NotFound,
    InternalServerError(sqlx::Error),
}

pub struct GetExerciseByLessonIdQueryHandler {
    repository: ExerciseRepository,
}

impl GetExerciseByLessonIdQueryHandler {
    pub fn new(repository: ExerciseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetExerciseByLessonIdQuery,
    ) -> Result<Vec<Exercise>, GetExerciseByLessonIdError> {
        let exercise = self
            .repository
            .get_exercise_by_lesson_id(query.id)
            .await
            .map_err(GetExerciseByLessonIdError::InternalServerError)?;

        if exercise.is_empty() {
            return Err(GetExerciseByLessonIdError::NotFound);
        }

        Ok(exercise)
    }
}
