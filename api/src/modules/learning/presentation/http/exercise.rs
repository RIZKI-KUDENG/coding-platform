use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::modules::learning::{
    application::use_cases::exercise::queries::{
        get_exercise_by_id::{GetExerciseByIdQuery, GetExerciseByIdQueryHandler},
        get_exercise_by_lesson_id::{
            GetExerciseByLessonIdQuery, GetExerciseByLessonIdQueryHandler,
        },
    },
    infrastructure::repositories::exercise_repository::ExerciseRepository,
    presentation::dtos::ExerciseResponse,
};
use crate::modules::shared::http::ApiError;
use crate::state::AppState;

pub async fn get_exercise_by_lesson_id(
    State(state): State<AppState>,
    Path(lesson_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetExerciseByLessonIdQueryHandler::new(ExerciseRepository::new(state.db.clone()));

    let exercises = handler
        .handle(GetExerciseByLessonIdQuery { id: lesson_id })
        .await?;

    Ok(Json(json!({
        "data": exercises.into_iter().map(ExerciseResponse::from).collect::<Vec<_>>(),
    })))
}

pub async fn get_exercise_by_id(
    State(state): State<AppState>,
    Path(exercise_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetExerciseByIdQueryHandler::new(ExerciseRepository::new(state.db.clone()));

    let exercise = handler
        .handle(GetExerciseByIdQuery { id: exercise_id })
        .await?;

    Ok(Json(json!({ "data": ExerciseResponse::from(exercise) })))
}
