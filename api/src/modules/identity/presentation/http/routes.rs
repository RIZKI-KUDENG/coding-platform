use axum::{Router, routing::post};

use super::auth;
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/register", post(auth::register))
}
