use axum::{
   Json,
   extract::{Path, State},
   response::IntoResponse,
   http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;
use crate::state::AppState;

use crate::modules::learning::application::use_cases::exercise::queries::get_exercise_by_id::{GetExerciseByIdError,GetExerciseByIdQuery,GetExerciseByIdQueryHandler};
use crate::modules::learning::application::use_cases::exercise::queries::get_exercise_by_lesson_id::{GetExerciseByLessonIdError,GetExerciseByLessonIdQuery,GetExerciseByLessonIdQueryHandler};
use crate::modules::learning::infrastructure::repositories::exercise_repository::ExerciseRepository;


pub async fn get_exercise_by_lesson_id(
    State(state): State<AppState>,
    Path(lesson_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = ExerciseRepository::new(state.db.clone());
    let handler = GetExerciseByLessonIdQueryHandler::new(repo);


    match handler.handle(GetExerciseByLessonIdQuery{id: lesson_id}).await {
        Ok(exercise) => (
            StatusCode::OK,
            Json(json!({
                "data": exercise
            }))
        ),
        Err(GetExerciseByLessonIdError::InternalServerError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string()
            }))
        ),
        Err(GetExerciseByLessonIdError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Exercise not found"
            }))
        ),
    }
}


pub async fn get_exercise_by_id(
    State(state): State<AppState>,
    Path(exercise_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = ExerciseRepository::new(state.db.clone());
    let handler = GetExerciseByIdQueryHandler::new(repo);


    match handler.handle(GetExerciseByIdQuery{id: exercise_id}).await {
        Ok(exercise) => (
            StatusCode::OK,
            Json(json!({
                "data": exercise
            }))
        ),
        Err(GetExerciseByIdError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string()
            }))
        ),
        Err(GetExerciseByIdError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Exercise not found"
            }))
        ),
    }
}
