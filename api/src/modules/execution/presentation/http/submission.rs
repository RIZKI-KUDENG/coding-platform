use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::modules::execution::application::use_cases::commands::execute_code_command::{
    ExecuteCodeCommand, ExecuteCodeCommandHandler,
};
use crate::modules::execution::infrastructure::repositories::submission_repository::SubmissionRepository;
use crate::modules::execution::infrastructure::runners::podman_runner::PodmanRunner;
use crate::modules::execution::presentation::dtos::submission_request::SubmissionRequest;
use crate::state::AppState;

pub async fn submit(
    State(state): State<AppState>,
    Path(exercise_id): Path<Uuid>,
    Json(request): Json<SubmissionRequest>,
) -> impl IntoResponse {
    let repo = SubmissionRepository::new(state.db.clone());
    let runner = PodmanRunner::new();
    let handler = ExecuteCodeCommandHandler::new(repo, runner);

    let command = ExecuteCodeCommand {
        user_id: Uuid::new_v4(),
        exercise_id,
        code: request.code,
        language: request.language,
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
                "error": {
                    "message": err.to_string()
                }
            })),
        ),
    }
}
