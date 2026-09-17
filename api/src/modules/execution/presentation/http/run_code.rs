use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

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
    // 1. Cek status fitur induk di database: Playground
    check_feature(&state, "playground", Some("Area Uji Coba (Playground)")).await?;

    // 2. Cek status sub-fitur bahasa runner di database: runner:<language> (misal: "runner:python")
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

    match handler.handle(command).await {
        Ok(result) => Ok((
            StatusCode::OK,
            Json(json!({
                "data": result
            })),
        )),
        Err(err) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": {
                    "message": err.to_string()
                }
            })),
        )),
    }
}
