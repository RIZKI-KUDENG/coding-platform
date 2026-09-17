use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::modules::execution::application::use_cases::playground::commands::run_code_command::{
    RunCodeCommand, RunCodeCommandHandler,
};
use crate::modules::execution::infrastructure::runners::podman_runner::PodmanRunner;
use crate::modules::execution::presentation::dtos::run_code_request::RunCodeRequest;

pub async fn run(Json(request): Json<RunCodeRequest>) -> impl IntoResponse {
    let runner = PodmanRunner::new();
    let handler = RunCodeCommandHandler::new(runner);

    let command = RunCodeCommand {
        language: request.language,
        code: request.code,
    };

    match handler.handle(command).await {
        Ok(result) => (
            StatusCode::OK,
            Json(json!({
                "data": result
            })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error" : {
                    "message": err.to_string()
                }
            })),
        ),
    }
}
