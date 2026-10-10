use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    modules::system::{
        application::use_cases::feature_flags::{
            commands::edit_feature_flags_command::{
                EditFeatureFlagsCommand, EditFeatureFlagsCommandHandler,
            },
            queries::{GetFeatureFlagsError, GetFeatureFlagsQuery, GetFeatureFlagsQueryHandler},
        },
        infrastructure::repositories::feature_flag_repository::FeatureFlagRepository,
    },
    state::AppState,
};
use serde_json::json;

pub async fn get_features(State(state): State<AppState>) -> impl IntoResponse {
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

pub async fn edit_feature_flags(
    State(state): State<AppState>,
    Json(command): Json<EditFeatureFlagsCommand>,
) -> impl IntoResponse {
    let repo = FeatureFlagRepository::new(state.db.clone());
    let handler = EditFeatureFlagsCommandHandler::new(repo);

    match handler.handle(command).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": "Fitur berhasil diperbarui."
            })),
        ),
        Err(err) => {
            eprintln!("Error editing feature flags: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": {
                        "code": "INTERNAL_SERVER_ERROR",
                        "message": "Gagal memperbarui status fitur."
                    }
                })),
            )
        }
    }
}
