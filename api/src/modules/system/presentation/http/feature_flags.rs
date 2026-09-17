use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json
};

use serde_json::json;
use crate::{
    modules::system::{
        application::use_cases::feature_flags::queries::{
            GetFeatureFlagsQuery,
            GetFeatureFlagsQueryHandler,
            GetFeatureFlagsError
        },
        infrastructure::repositories::feature_flag_repository::FeatureFlagRepository,
    },
    state::AppState
};

pub async fn get_features(
    State(state): State<AppState>
) -> impl IntoResponse{
    let repo = FeatureFlagRepository::new(state.db.clone());

    let handler = GetFeatureFlagsQueryHandler::new(repo);

    match handler.handle(GetFeatureFlagsQuery).await {
        Ok(flags) => (
            StatusCode::OK,
            Json(json!({
                "data": flags
            })),
        ),
        Err(GetFeatureFlagsError::DatabaseError(err)) => {
            eprintln!("Database error fetching feature flags {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": {
                        "code": "INTERNAL_SERVER_ERROR",
                        "message": "Gagal membaca status fitur."
                    }
                })),
            )
        }
    }
}
