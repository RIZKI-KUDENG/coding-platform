use super::run_code;
use super::submission;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn execution_routes() -> Router<AppState> {
    Router::new()
        .nest(
            "/api/v1/exercises/{exercise_id}",
            Router::new().route("/submissions", post(submission::submit)),
        )
        .route("/api/v1/playground/run", post(run_code::run))
}
