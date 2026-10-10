use crate::modules::shared::error::InfrastructureError;
use uuid::Uuid;

use crate::modules::learning::domain::entities::exercise_test_case::ExerciseTestCase;
use crate::modules::learning::infrastructure::repositories::exercise_test_case_repository::ExerciseTestCaseRepository;

pub struct GetTestCaseByExerciseIdQuery {
    pub exercise_id: Uuid,
}

#[derive(Debug)]
pub enum GetTestCaseByExerciseIdError {
    NotFound,
    DatabaseError(InfrastructureError),
}

#[derive(Clone)]
pub struct GetTestCaseByExerciseIdQueryHandler {
    repository: ExerciseTestCaseRepository,
}

impl GetTestCaseByExerciseIdQueryHandler {
    pub fn new(repository: ExerciseTestCaseRepository) -> Self {
        Self { repository }
    }

    /// Public contract for other modules: lets them build the handler without
    /// touching this module's infrastructure.
    pub fn from_pool(pool: sqlx::PgPool) -> Self {
        Self::new(ExerciseTestCaseRepository::new(pool))
    }

    pub async fn handle(
        &self,
        query: GetTestCaseByExerciseIdQuery,
    ) -> Result<Vec<ExerciseTestCase>, GetTestCaseByExerciseIdError> {
        let test_cases = self
            .repository
            .get_test_case_by_exercise_id(query.exercise_id)
            .await
            .map_err(|e| GetTestCaseByExerciseIdError::DatabaseError(e.into()))?;

        if test_cases.is_empty() {
            return Err(GetTestCaseByExerciseIdError::NotFound);
        }
        Ok(test_cases)
    }
}
