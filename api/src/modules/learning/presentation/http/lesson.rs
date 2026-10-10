use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::modules::learning::{
    application::use_cases::lesson::queries::{
        get_lesson_by_course_id::{GetLessonByCourseIdQuery, GetLessonByCourseIdQueryHandler},
        get_lesson_by_id::{GetLessonByIdQuery, GetLessonByIdQueryHandler},
        get_lesson_by_slug::{GetLessonBySlugQuery, GetLessonBySlugQueryHandler},
    },
    infrastructure::repositories::lesson_repository::LessonRepository,
    presentation::dtos::LessonResponse,
};
use crate::modules::shared::http::ApiError;
use crate::state::AppState;

pub async fn get_lesson_by_course_id(
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetLessonByCourseIdQueryHandler::new(LessonRepository::new(state.db.clone()));

    let lessons = handler
        .handle(GetLessonByCourseIdQuery { id: course_id })
        .await?;

    Ok(Json(json!({
        "data": lessons.into_iter().map(LessonResponse::from).collect::<Vec<_>>(),
    })))
}

pub async fn get_lesson_by_id(
    State(state): State<AppState>,
    Path(lesson_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetLessonByIdQueryHandler::new(LessonRepository::new(state.db.clone()));

    let lesson = handler.handle(GetLessonByIdQuery { id: lesson_id }).await?;

    Ok(Json(json!({ "data": LessonResponse::from(lesson) })))
}

pub async fn get_lesson_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetLessonBySlugQueryHandler::new(LessonRepository::new(state.db.clone()));

    let lesson = handler.handle(GetLessonBySlugQuery { slug }).await?;

    Ok(Json(json!({ "data": LessonResponse::from(lesson) })))
}
