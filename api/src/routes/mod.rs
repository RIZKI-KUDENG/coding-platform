use axum::{Router, routing::get};

use crate::modules::execution::execution_routes;
use crate::modules::identity::identity_routes;
use crate::modules::system::system_routes;

use crate::state::AppState;

mod health;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health::health_check))
        .merge(identity_routes())
        .merge(execution_routes())
        .merge(system_routes())
        .with_state(state)
}
