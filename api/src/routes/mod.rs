use axum::{
    routing::get,
    Router,
};

use crate::state::AppState;

mod health;


pub fn create_router(state: AppState) -> Router{
    Router::new()
        .route("/api/v1/health", get(health::health_check))
        .with_state(state)
}
