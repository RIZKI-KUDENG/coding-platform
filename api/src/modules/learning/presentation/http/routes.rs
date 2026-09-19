use super::course;
use crate::state::AppState;
use axum::{routing::get, Router};

pub fn learning_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/courses", get(course::get_all_course))
        .route("/api/v1/courses/:id", get(course::get_course_by_id))
}
