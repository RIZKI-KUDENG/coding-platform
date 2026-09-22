use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
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
use crate::modules::learning::{
    ExerciseTestCaseRepository, GetTestCaseByExerciseIdQueryHandler,
};

pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(exercise_id): Path<Uuid>,
    Json(request): Json<SubmissionRequest>,
) -> Result<impl IntoResponse, Response> {
    let runner_key = format!("runner:{}", request.language);
    check_feature(
        &state,
        &runner_key,
        Some(&format!("Runner {}", request.language)),
    )
    .await?;

    let repo = SubmissionRepository::new(state.db.clone());
    let test_case_repo = ExerciseTestCaseRepository::new(state.db.clone());
    let test_case_handler = GetTestCaseByExerciseIdQueryHandler::new(test_case_repo);
    let runner = PodmanRunner::new();
    let handler = ExecuteCodeCommandHandler::new(repo, test_case_handler, runner);

    let command = ExecuteCodeCommand {
        user_id: user.user_id,
        exercise_id,
        code: request.code,
        language: request.language,
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
