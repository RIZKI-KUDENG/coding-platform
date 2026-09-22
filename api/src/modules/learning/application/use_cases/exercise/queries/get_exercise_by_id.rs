use uuid::Uuid;

use crate::modules::learning::domain::entities::exercise::Exercise;
use crate::modules::learning::infrastructure::repositories::exercise_repository::ExerciseRepository;

pub struct GetExerciseByIdQuery {
    pub id: Uuid,
}

#[derive(Debug)]
pub enum GetExerciseByIdError {
    NotFound,
    DatabaseError(sqlx::Error),
}

pub struct GetExerciseByIdQueryHandler {
    repository: ExerciseRepository,
}

impl GetExerciseByIdQueryHandler {
    pub fn new(repository: ExerciseRepository) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetExerciseByIdQuery,
    ) -> Result<Option<Exercise>, GetExerciseByIdError> {
        let exercise = self
            .repository
            .get_exercise_by_id(query.id)
            .await
            .map_err(GetExerciseByIdError::DatabaseError)?;

        if exercise.is_none() {
            return Err(GetExerciseByIdError::NotFound);
        }
        Ok(exercise)
    }
}
