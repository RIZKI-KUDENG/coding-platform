use super::feature_flags;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn system_routes() -> Router<AppState> {
    Router::new().route("/api/v1/features", get(feature_flags::get_features))
}
