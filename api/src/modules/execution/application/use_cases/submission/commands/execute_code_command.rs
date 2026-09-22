use uuid::Uuid;

use crate::modules::execution::application::ports::code_runner::{
    CodeRunner, RunRequest, RunResult,
};
use crate::modules::execution::domain::entities::submission::SubmissionStatus;
use crate::modules::execution::infrastructure::repositories::submission_repository::SubmissionRepository;
use crate::modules::learning::{GetTestCaseByExerciseIdQuery, GetTestCaseByExerciseIdQueryHandler};

pub struct ExecuteCodeCommand {
    pub user_id: Uuid,
    pub exercise_id: Uuid,
    pub code: String,
    pub language: String,
}

pub struct ExecuteCodeCommandHandler<C>
where
    C: CodeRunner,
{
    submission_repository: SubmissionRepository,
    test_case_handler: GetTestCaseByExerciseIdQueryHandler,
    code_runner: C,
}

impl<C> ExecuteCodeCommandHandler<C>
where
    C: CodeRunner,
{
    pub fn new(
        submission_repository: SubmissionRepository,
        test_case_handler: GetTestCaseByExerciseIdQueryHandler,
        code_runner: C,
    ) -> Self {
        Self {
            submission_repository,
            test_case_handler,
            code_runner,
        }
    }

    pub async fn handle(
        &self,
        command: ExecuteCodeCommand,
    ) -> Result<RunResult, Box<dyn std::error::Error>> {
        let test_cases = self
            .test_case_handler
            .handle(GetTestCaseByExerciseIdQuery {
                exercise_id: command.exercise_id,
            })
            .await
            .unwrap_or_default();

        let mut all_passed = true;
        let mut total_duration_ms: u64 = 0;
        let mut final_status = SubmissionStatus::Passed;
        let mut last_result = RunResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
            duration_ms: 0,
        };

        if test_cases.is_empty() {
            let request = RunRequest {
                language: command.language.clone(),
                code: command.code.clone(),
                input: None,
            };
            let result = self.code_runner.run(request).await?;
            total_duration_ms = result.duration_ms;
            final_status = if result.exit_code == Some(124) {
                SubmissionStatus::Timeout
            } else if !result.success {
                SubmissionStatus::RuntimeError
            } else {
                SubmissionStatus::Passed
            };
            last_result = result;
        } else {
            for tc in test_cases {
                let request = RunRequest {
                    language: command.language.clone(),
                    code: command.code.clone(),
                    input: if tc.input.is_empty() {
                        None
                    } else {
                        Some(tc.input.clone())
                    },
                };

                let result = self.code_runner.run(request).await?;
                total_duration_ms += result.duration_ms;
                last_result = result.clone();

                if result.exit_code == Some(124) {
                    final_status = SubmissionStatus::Timeout;
                    all_passed = false;
                    break;
                } else if !result.success {
                    final_status = SubmissionStatus::RuntimeError;
                    all_passed = false;
                    break;
                }

                let actual = result.stdout.trim();
                let expected = tc.expected_output.trim();

                if actual != expected {
                    final_status = SubmissionStatus::Failed;
                    all_passed = false;
                    break;
                }
            }

            if all_passed {
                final_status = SubmissionStatus::Passed;
            }
        }

        let submission = self
            .submission_repository
            .create_submission(
                command.user_id,
                command.exercise_id,
                &command.code,
                &command.language,
            )
            .await?;

        self.submission_repository
            .update_status(
                submission.id,
                final_status.clone(),
                Some(total_duration_ms as i32),
            )
            .await?;

        last_result.success = final_status == SubmissionStatus::Passed;
        last_result.duration_ms = total_duration_ms;

        Ok(last_result)
    }
}
