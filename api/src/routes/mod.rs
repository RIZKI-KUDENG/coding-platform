use axum::{Router, routing::get};

use crate::modules::identity::presentation::http::routes::auth_routes;
use crate::state::AppState;

mod health;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health::health_check))
        .nest("/api/v1/auth", auth_routes())
        .with_state(state)
}
