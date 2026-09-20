use super::course;
use super::lesson;
use crate::state::AppState;
use axum::{routing::get, Router};

pub fn learning_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/courses", get(course::get_all_course))
        .route("/api/v1/courses/{id}", get(course::get_course_by_id))
        .route("/api/v1/courses/slug/{slug}", get(course::get_course_by_slug))
        .route("/api/v1/courses/{course_id}/lessons", get(lesson::get_lesson_by_course_id))
        .route("/api/v1/lessons/{lesson_id}", get(lesson::get_lesson_by_id))
        .route("/api/v1/lessons/slug/{slug}", get(lesson::get_lesson_by_slug))
}
