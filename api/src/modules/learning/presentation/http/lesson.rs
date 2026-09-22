use crate::modules::learning::application::use_cases::lesson::queries::get_lesson_by_course_id::{
    GetLessonByCourseIdError, GetLessonByCourseIdQuery, GetLessonByCourseIdQueryHandler,
};
use crate::modules::learning::application::use_cases::lesson::queries::get_lesson_by_id::{
    GetLessonByIdError, GetLessonByIdQuery, GetLessonByIdQueryHandler,
};
use crate::modules::learning::application::use_cases::lesson::queries::get_lesson_by_slug::{
    GetLessonBySlugError, GetLessonBySlugQuery, GetLessonBySlugQueryHandler,
};
use crate::modules::learning::infrastructure::repositories::lesson_repository::LessonRepository;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn get_lesson_by_course_id(
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = LessonRepository::new(state.db.clone());
    let handler = GetLessonByCourseIdQueryHandler::new(repo);

    match handler
        .handle(GetLessonByCourseIdQuery { id: course_id })
        .await
    {
        Ok(lesson) => (
            StatusCode::OK,
            Json(json!({
                "data": lesson
            })),
        ),
        Err(GetLessonByCourseIdError::DatabaseError(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string()
            })),
        ),
    }
}

pub async fn get_lesson_by_id(
    State(state): State<AppState>,
    Path(lesson_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = LessonRepository::new(state.db.clone());
    let handler = GetLessonByIdQueryHandler::new(repo);

    match handler.handle(GetLessonByIdQuery { id: lesson_id }).await {
        Ok(lesson) => (
            StatusCode::OK,
            Json(json!({
                "data": lesson
            })),
        ),
        Err(GetLessonByIdError::DatabaseError(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string()
            })),
        ),
        Err(GetLessonByIdError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Lesson not found"
            })),
        ),
    }
}

pub async fn get_lesson_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let repo = LessonRepository::new(state.db.clone());
    let handler = GetLessonBySlugQueryHandler::new(repo);

    match handler.handle(GetLessonBySlugQuery { slug }).await {
        Ok(lesson) => (
            StatusCode::OK,
            Json(json!({
                "data": lesson
            })),
        ),
        Err(GetLessonBySlugError::DatabaseError(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string()
            })),
        ),
        Err(GetLessonBySlugError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Lesson not found"
            })),
        ),
    }
}
