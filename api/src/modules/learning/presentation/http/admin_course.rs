use crate::modules::learning::{
    application::use_cases::admin::course::{
        commands::{
            create_course_admin::{CreateCourseCommand, CreateCourseCommandHandler},
            delete_course_admin::{
                DeleteCourseCommand, DeleteCourseCommandHandler, DeleteCourseError,
            },
            edit_course_admin::{EditCourseCommand, EditCourseCommandHandler, EditCourseError},
        },
        queries::get_course_admin::GetCourseAdminQueryHandler,
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
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct EditCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
}

pub async fn get_all_courses_admin(State(state): State<AppState>) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = GetCourseAdminQueryHandler::new(repo);

    match handler.handle().await {
        Ok(courses) => (
            StatusCode::OK,
            Json(json!({
                "data": courses,
            })),
        ),
        Err(err) if err.to_string() == "No courses found" => (
            StatusCode::OK,
            Json(json!({
                "data": [],
            })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}

pub async fn create_course_admin(
    State(state): State<AppState>,
    Json(payload): Json<CreateCourseRequest>,
) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = CreateCourseCommandHandler::new(repo);

    let command = CreateCourseCommand {
        title: payload.title,
        slug: payload.slug,
        description: payload.description,
        status: payload.status,
    };

    match handler.handle(command).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "data": {
                    "message": "Course created successfully"
                }
            })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}

pub async fn edit_course_admin(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<EditCourseRequest>,
) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = EditCourseCommandHandler::new(repo);

    let command = EditCourseCommand {
        id,
        title: payload.title,
        slug: payload.slug,
        description: payload.description,
        status: payload.status,
    };

    match handler.handle(command).await {
        Ok(success) => (
            StatusCode::OK,
            Json(json!({
                "data": {
                    "success": success
                }
            })),
        ),
        Err(EditCourseError::CourseNotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Course not found",
            })),
        ),
        Err(EditCourseError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}

pub async fn delete_course_admin(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = DeleteCourseCommandHandler::new(repo);

    match handler.handle(DeleteCourseCommand { course_id: id }).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "data": {
                    "success": true
                }
            })),
        ),
        Err(DeleteCourseError::CourseNotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Course not found",
            })),
        ),
        Err(DeleteCourseError::DatabaseError(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": err.to_string(),
            })),
        ),
    }
}
