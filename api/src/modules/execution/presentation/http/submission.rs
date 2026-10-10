use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::modules::shared::http::ApiError;

use crate::modules::execution::presentation::dtos::RunResultResponse;
use uuid::Uuid;

use crate::modules::shared::authentication::authenticated_user::AuthenticatedUser;
use crate::modules::shared::check_feature;

use crate::modules::execution::application::use_cases::submission::commands::execute_code_command::{
    ExecuteCodeCommand, ExecuteCodeCommandHandler,
};
use crate::modules::execution::infrastructure::repositories::submission_repository::SubmissionRepository;
use crate::modules::execution::infrastructure::runners::podman_runner::PodmanRunner;
use crate::modules::execution::presentation::dtos::submission_request::SubmissionRequest;
use crate::state::AppState;
use crate::modules::learning::GetTestCaseByExerciseIdQueryHandler;

pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(exercise_id): Path<Uuid>,
    Json(request): Json<SubmissionRequest>,
) -> Result<impl IntoResponse, Response> {
    // Cek status dinamis sub-fitur bahasa runner berdasarkan request payload
    let runner_key = format!("runner:{}", request.language);
    check_feature(
        &state,
        &runner_key,
        Some(&format!("Runner {}", request.language)),
    )
    .await?;

    let repo = SubmissionRepository::new(state.db.clone());
    let test_case_handler = GetTestCaseByExerciseIdQueryHandler::from_pool(state.db.clone());
    let runner = PodmanRunner::new();
    let handler = ExecuteCodeCommandHandler::new(repo, test_case_handler, runner);

    let command = ExecuteCodeCommand {
        user_id: user.user_id,
        exercise_id,
        code: request.code,
        language: request.language,
    };

    let result = handler
        .handle(command)
        .await
        .map_err(|e| ApiError::from(e).into_response())?;

    Ok(Json(json!({
        "data": RunResultResponse::from(result)
    })))
}
