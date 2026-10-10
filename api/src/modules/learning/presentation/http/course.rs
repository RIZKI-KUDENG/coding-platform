use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::modules::learning::{
    application::use_cases::course::queries::{
        get_all_course::{GetAllCourseQuery, GetAllCourseQueryHandler},
        get_course_by_id::{GetCourseByIdQuery, GetCourseByIdQueryHandler},
        get_course_by_slug::{GetCourseBySlugQuery, GetCourseBySlugQueryHandler},
    },
    infrastructure::repositories::course_repository::CourseRepository,
    presentation::dtos::CourseResponse,
};
use crate::modules::shared::http::ApiError;
use crate::state::AppState;

pub async fn get_all_course(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let handler = GetAllCourseQueryHandler::new(CourseRepository::new(state.db.clone()));

    let courses = handler.handle(GetAllCourseQuery).await?;

    Ok(Json(json!({
        "data": courses.into_iter().map(CourseResponse::from).collect::<Vec<_>>(),
    })))
}

pub async fn get_course_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetCourseByIdQueryHandler::new(CourseRepository::new(state.db.clone()));

    let course = handler.handle(GetCourseByIdQuery { id }).await?;

    Ok(Json(json!({ "data": CourseResponse::from(course) })))
}

pub async fn get_course_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = GetCourseBySlugQueryHandler::new(CourseRepository::new(state.db.clone()));

    let course = handler.handle(GetCourseBySlugQuery { slug }).await?;

    Ok(Json(json!({ "data": CourseResponse::from(course) })))
}
