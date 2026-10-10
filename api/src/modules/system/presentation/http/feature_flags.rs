use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    modules::system::{
        application::use_cases::feature_flags::{
            commands::edit_feature_flags_command::{
                EditFeatureFlagsCommand, EditFeatureFlagsCommandHandler,
            },
            queries::{GetFeatureFlagsError, GetFeatureFlagsQuery, GetFeatureFlagsQueryHandler},
        },
        infrastructure::repositories::feature_flag_repository::FeatureFlagRepository,
        presentation::dtos::{EditFeatureFlagsRequest, FeatureFlagResponse, status_map_response},
    },
    state::AppState,
};
use serde_json::json;

use crate::modules::shared::http::ApiError;

pub async fn get_features(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let handler = GetFeatureFlagsQueryHandler::new(FeatureFlagRepository::new(state.db.clone()));

    let flags = handler
        .handle(GetFeatureFlagsQuery)
        .await
        .map_err(|err| match err {
            GetFeatureFlagsError::DatabaseError(e) => ApiError::internal(&e),
        })?;

    Ok(Json(json!({ "data": status_map_response(flags) })))
}

pub async fn get_admin_features(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let repo = FeatureFlagRepository::new(state.db.clone());

    let flags = repo
        .get_all()
        .await
        .map_err(|e| ApiError::internal(&e.into()))?;

    Ok(Json(json!({
        "data": flags.into_iter().map(FeatureFlagResponse::from).collect::<Vec<_>>()
    })))
}

pub async fn edit_feature_flags(
    State(state): State<AppState>,
    Json(request): Json<EditFeatureFlagsRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = EditFeatureFlagsCommandHandler::new(FeatureFlagRepository::new(state.db.clone()));

    handler
        .handle(EditFeatureFlagsCommand::from(request))
        .await
        .map_err(|e| ApiError::internal(&e))?;

    Ok(Json(json!({ "message": "Fitur berhasil diperbarui." })))
}
