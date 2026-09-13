use axum::{Router, routing::post};
use super::submission;
use crate::state::AppState;


pub fn execution_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/exercises/{exercise_id}",
        Router::new().route("/submissions", post(submission::submit)),
    )
}
