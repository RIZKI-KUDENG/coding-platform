use axum::{Router, routing::post};

use super::auth;
use crate::state::AppState;

pub fn identity_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1/auth",
        Router::new()
            .route("/register", post(auth::register))
            .route("/login", post(auth::login)),
    )
}
