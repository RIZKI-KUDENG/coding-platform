use super::course;
use super::lesson;
use crate::modules::shared::feature_flags::check_feature;
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

async fn require_courses_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, "courses", Some("Jalur Belajar")).await?;
    Ok(next.run(req).await)
}

pub fn learning_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/v1/courses", get(course::get_all_course))
        .route("/api/v1/courses/{id}", get(course::get_course_by_id))
        .route(
            "/api/v1/courses/slug/{slug}",
            get(course::get_course_by_slug),
        )
        .route(
            "/api/v1/courses/{course_id}/lessons",
            get(lesson::get_lesson_by_course_id),
        )
        .route(
            "/api/v1/lessons/{lesson_id}",
            get(lesson::get_lesson_by_id),
        )
        .route(
            "/api/v1/lessons/slug/{slug}",
            get(lesson::get_lesson_by_slug),
        )
        .route_layer(middleware::from_fn_with_state(
            state,
            require_courses_feature,
        ))
}
