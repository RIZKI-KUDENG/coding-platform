use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    sqlx::query("SELECT 1").execute(&state.db).await.unwrap();

    Json(json!({ "status": "ok" }))
}
