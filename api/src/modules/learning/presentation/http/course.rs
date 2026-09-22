use crate::modules::learning::{
    application::use_cases::course::queries::{
        get_all_course::{GetAllCourseError, GetAllCourseQuery, GetAllCourseQueryHandler},
        get_course_by_id::{GetCourseByIdError, GetCourseByIdQuery, GetCourseByIdQueryHandler},
        get_course_by_slug::{
            GetCourseBySlugError, GetCourseBySlugQuery, GetCourseBySlugQueryHandler,
        },
    },
    infrastructure::repositories::course_repository::CourseRepository,
};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn get_all_course(State(state): State<AppState>) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = GetAllCourseQueryHandler::new(repo);

    match handler.handle(GetAllCourseQuery).await {
        Ok(course) => (
            StatusCode::OK,
            Json(json!({
                "data": course,
            })),
        ),
        Err(GetAllCourseError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}

pub async fn get_course_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = GetCourseByIdQueryHandler::new(repo);

    match handler.handle(GetCourseByIdQuery { id }).await {
        Ok(course) => (
            StatusCode::OK,
            Json(json!({
                "data": course,
            })),
        ),
        Err(GetCourseByIdError::CourseNotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Course not found",
            })),
        ),
        Err(GetCourseByIdError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}

pub async fn get_course_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = GetCourseBySlugQueryHandler::new(repo);

    match handler.handle(GetCourseBySlugQuery { slug }).await {
        Ok(course) => (
            StatusCode::OK,
            Json(json!({
                "data": course,
            })),
        ),
        Err(GetCourseBySlugError::CourseNotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Course not found",
            })),
        ),
        Err(GetCourseBySlugError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}
