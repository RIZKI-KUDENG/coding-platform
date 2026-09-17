use axum::{routing::get, Router};
use super::feature_flags;
use crate::state::AppState;

pub fn system_routes() -> Router<AppState> {
    Router::new().route("/api/v1/features", get(feature_flags::get_features))
}
