use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::learning::domain::entities::exercise::Exercise;
use crate::modules::learning::infrastructure::repositories::exercise_repository::ExerciseRepository;

pub struct GetExerciseByIdQuery {
    pub id: Uuid,
}

#[derive(Debug)]
pub enum GetExerciseByIdError {
    NotFound,
    DatabaseError(InfrastructureError),
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
    ) -> Result<Exercise, GetExerciseByIdError> {
        self.repository
            .get_exercise_by_id(query.id)
            .await
            .map_err(|e| GetExerciseByIdError::DatabaseError(e.into()))?
            .ok_or(GetExerciseByIdError::NotFound)
    }
}
