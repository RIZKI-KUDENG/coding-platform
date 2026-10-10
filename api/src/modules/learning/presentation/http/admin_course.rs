use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::modules::learning::{
    application::use_cases::admin::course::{
        commands::{
            create_course_admin::{CreateCourseCommand, CreateCourseCommandHandler},
            delete_course_admin::{DeleteCourseCommand, DeleteCourseCommandHandler},
            edit_course_admin::{EditCourseCommand, EditCourseCommandHandler},
        },
        queries::get_course_admin::GetCourseAdminQueryHandler,
    },
    infrastructure::repositories::course_repository::CourseRepository,
    presentation::dtos::{CourseResponse, CreateCourseRequest, EditCourseRequest},
};
use crate::modules::shared::http::ApiError;
use crate::state::AppState;

pub async fn get_all_courses_admin(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetCourseAdminQueryHandler::new(CourseRepository::new(state.db.clone()));

    let courses = handler.handle().await.map_err(|e| ApiError::internal(&e))?;

    Ok(Json(json!({
        "data": courses.into_iter().map(CourseResponse::from).collect::<Vec<_>>(),
    })))
}

pub async fn create_course_admin(
    State(state): State<AppState>,
    Json(payload): Json<CreateCourseRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = CreateCourseCommandHandler::new(CourseRepository::new(state.db.clone()));

    handler
        .handle(CreateCourseCommand {
            title: payload.title,
            slug: payload.slug,
            description: payload.description,
            status: payload.status,
        })
        .await
        .map_err(|e| ApiError::internal(&e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "data": { "message": "Course created successfully" }
        })),
    ))
}

pub async fn edit_course_admin(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<EditCourseRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = EditCourseCommandHandler::new(CourseRepository::new(state.db.clone()));

    let success = handler
        .handle(EditCourseCommand {
            id,
            title: payload.title,
            slug: payload.slug,
            description: payload.description,
            status: payload.status,
        })
        .await?;

    Ok(Json(json!({ "data": { "success": success } })))
}

pub async fn delete_course_admin(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = DeleteCourseCommandHandler::new(CourseRepository::new(state.db.clone()));

    handler
        .handle(DeleteCourseCommand { course_id: id })
        .await?;

    Ok(Json(json!({ "data": { "success": true } })))
}
