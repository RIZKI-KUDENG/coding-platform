use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::modules::shared::http::ApiError;

use crate::modules::execution::presentation::dtos::RunResultResponse;

use crate::{
    modules::{
        execution::{
            application::use_cases::playground::commands::run_code_command::{
                RunCodeCommand, RunCodeCommandHandler,
            },
            infrastructure::runners::podman_runner::PodmanRunner,
            presentation::dtos::run_code_request::RunCodeRequest,
        },
        shared::check_feature,
    },
    state::AppState,
};

pub async fn run(
    State(state): State<AppState>,
    Json(request): Json<RunCodeRequest>,
) -> Result<impl IntoResponse, Response> {
    // Cek status dinamis sub-fitur bahasa runner berdasarkan request payload
    let runner_key = format!("runner:{}", request.language);
    check_feature(
        &state,
        &runner_key,
        Some(&format!("Runner {}", request.language)),
    )
    .await?;

    let runner = PodmanRunner::new();
    let handler = RunCodeCommandHandler::new(runner);

    let command = RunCodeCommand {
        language: request.language,
        code: request.code,
    };

    let result = handler
        .handle(command)
        .await
        .map_err(|e| ApiError::from(e).into_response())?;

    Ok(Json(json!({
        "data": RunResultResponse::from(result)
    })))
}
