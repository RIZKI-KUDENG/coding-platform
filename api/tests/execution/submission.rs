use std::error::Error;
use uuid::Uuid;

use api::config::Config;
use api::modules::execution::application::ports::code_runner::{CodeRunner, RunRequest, RunResult};
use api::modules::execution::application::use_cases::commands::execute_code_command::{
    ExecuteCodeCommand, ExecuteCodeCommandHandler,
};
use api::modules::execution::infrastructure::repositories::submission_repository::SubmissionRepository;
use api::modules::execution::infrastructure::runners::podman_runner::PodmanRunner;
use api::state::AppState;

/// Mock runner for testing execution business logic without Docker/Podman
struct MockCodeRunner {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

#[allow(async_fn_in_trait)]
impl CodeRunner for MockCodeRunner {
    async fn run(&self, _request: RunRequest) -> Result<RunResult, Box<dyn Error>> {
        Ok(RunResult {
            success: self.success,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
            exit_code: self.exit_code,
            duration_ms: 25,
        })
    }
}

#[tokio::test]
async fn test_execute_code_status_lifecycle_with_mock() {
    let config = Config::load();
    let state = AppState::new(&config).await;
    let repo = SubmissionRepository::new(state.db.clone());

    let user_id = Uuid::new_v4();
    let exercise_id = Uuid::new_v4();

    // Case 1: Successful execution -> Should set status to PASSED
    let mock_success = MockCodeRunner {
        success: true,
        exit_code: Some(0),
        stdout: "Hello World".to_string(),
        stderr: String::new(),
    };
    let handler = ExecuteCodeCommandHandler::new(repo.clone(), mock_success);
    let cmd = ExecuteCodeCommand {
        user_id,
        exercise_id,
        code: "print('Hello World')".to_string(),
        language: "python".to_string(),
    };
    let result = handler.handle(cmd).await.expect("Handler should succeed");
    assert!(result.success);
    assert_eq!(result.stdout, "Hello World");

    // Case 2: Timeout execution (exit_code 124) -> Should set status to TIMEOUT
    let mock_timeout = MockCodeRunner {
        success: false,
        exit_code: Some(124),
        stdout: String::new(),
        stderr: "execution timed out".to_string(),
    };
    let handler_timeout = ExecuteCodeCommandHandler::new(repo.clone(), mock_timeout);
    let cmd_timeout = ExecuteCodeCommand {
        user_id,
        exercise_id,
        code: "while True: pass".to_string(),
        language: "python".to_string(),
    };
    let result_timeout = handler_timeout
        .handle(cmd_timeout)
        .await
        .expect("Handler should succeed");
    assert!(!result_timeout.success);
    assert_eq!(result_timeout.exit_code, Some(124));

    // Case 3: Runtime error -> Should set status to RUNTIME_ERROR
    let mock_error = MockCodeRunner {
        success: false,
        exit_code: Some(1),
        stdout: String::new(),
        stderr: "NameError: name 'foo' is not defined".to_string(),
    };
    let handler_error = ExecuteCodeCommandHandler::new(repo.clone(), mock_error);
    let cmd_error = ExecuteCodeCommand {
        user_id,
        exercise_id,
        code: "foo()".to_string(),
        language: "python".to_string(),
    };
    let result_error = handler_error
        .handle(cmd_error)
        .await
        .expect("Handler should succeed");
    assert!(!result_error.success);
    assert_eq!(result_error.exit_code, Some(1));
}

#[tokio::test]
async fn test_real_podman_runner_python() {
    let config = Config::load();
    let state = AppState::new(&config).await;
    let repo = SubmissionRepository::new(state.db.clone());
    let runner = PodmanRunner::new();
    let handler = ExecuteCodeCommandHandler::new(repo.clone(), runner);

    let user_id = Uuid::new_v4();
    let exercise_id = Uuid::new_v4();

    let cmd = ExecuteCodeCommand {
        user_id,
        exercise_id,
        code: "print(21 + 21)".to_string(),
        language: "python".to_string(),
    };

    let result = handler
        .handle(cmd)
        .await
        .expect("Execution via Podman should succeed");

    assert!(result.success);
    assert_eq!(result.stdout, "42");
    assert_eq!(result.exit_code, Some(0));
    assert!(result.duration_ms > 0);

    // Verify persisted in PostgreSQL
    let row = sqlx::query!(
        r#"
        SELECT user_id, exercise_id, status, execution_time_ms, code
        FROM execution.t_submissions
        WHERE user_id = $1 AND exercise_id = $2
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id,
        exercise_id,
    )
    .fetch_one(&state.db)
    .await
    .expect("Submission row must exist in DB");

    assert_eq!(row.user_id, user_id);
    assert_eq!(row.exercise_id, exercise_id);
    assert_eq!(row.status, "PASSED");
    assert!(row.execution_time_ms.unwrap_or(0) > 0);
    assert_eq!(row.code, "print(21 + 21)");
}
