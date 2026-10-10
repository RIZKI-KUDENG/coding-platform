use super::feature_flags;
use crate::modules::identity::require_admin;
use crate::state::AppState;
use axum::{
    Router, middleware,
    routing::{get, put},
};

pub fn system_routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new().route("/api/v1/features", get(feature_flags::get_features));

    let admin_routes = Router::new()
        .route("/api/v1/features", put(feature_flags::edit_feature_flags))
        .route(
            "/api/v1/admin/features",
            get(feature_flags::get_admin_features).put(feature_flags::edit_feature_flags),
        )
        .route_layer(middleware::from_fn_with_state(state, require_admin));

    public_routes.merge(admin_routes)
}
