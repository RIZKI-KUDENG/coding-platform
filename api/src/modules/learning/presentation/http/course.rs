use axum::{
   Json,
   extract::State,
   response::IntoResponse,
   http::StatusCode,
};
use serde_json::json;
use crate::state::AppState;
use crate::modules::learning::{
    application::use_cases::course::queries::get_all_course::{
        GetAllCourseError, GetAllCourseQuery, GetAllCourseQueryHandler
    },
    infrastructure::repositories::course_repository::CourseRepository
};



pub async fn get_all_course(State(state): State<AppState>) -> impl IntoResponse {
    let repo = CourseRepository::new(state.db.clone());
    let handler = GetAllCourseQueryHandler::new(repo);


    match handler.handle(GetAllCourseQuery).await{
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
