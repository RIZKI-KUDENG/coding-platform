use super::course;
use super::exercise;
use super::lesson;
use crate::modules::shared::feature_flags::check_feature;
use crate::state::AppState;
use axum::{
    Router,
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::get,
};

async fn require_courses_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, "courses", Some("Jalur Belajar")).await?;
    Ok(next.run(req).await)
}

async fn require_lessons_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, "courses:lessons", Some("Materi Pelajaran")).await?;
    Ok(next.run(req).await)
}

async fn require_exercise_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, "courses:exercise", Some("Latihan Koding")).await?;
    Ok(next.run(req).await)
}

pub fn learning_routes(state: AppState) -> Router<AppState> {
    let course_routes = Router::new()
        .route("/api/v1/courses", get(course::get_all_course))
        .route("/api/v1/courses/{id}", get(course::get_course_by_id))
        .route(
            "/api/v1/courses/slug/{slug}",
            get(course::get_course_by_slug),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_courses_feature,
        ));

    let lesson_routes = Router::new()
        .route(
            "/api/v1/courses/{course_id}/lessons",
            get(lesson::get_lesson_by_course_id),
        )
        .route("/api/v1/lessons/{lesson_id}", get(lesson::get_lesson_by_id))
        .route(
            "/api/v1/lessons/slug/{slug}",
            get(lesson::get_lesson_by_slug),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_lessons_feature,
        ));

    let exercise_route = Router::new()
        .route(
            "/api/v1/lessons/{lesson_id}/exercise",
            get(exercise::get_exercise_by_lesson_id),
        )
        .route(
            "/api/v1/exercise/{exercise_id}",
            get(exercise::get_exercise_by_id),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_exercise_feature,
        ));

    course_routes.merge(lesson_routes).merge(exercise_route)
}
