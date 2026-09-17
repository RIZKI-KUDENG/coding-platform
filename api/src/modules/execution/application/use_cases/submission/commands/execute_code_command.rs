use uuid::Uuid;

use crate::modules::execution::application::ports::code_runner::{
    CodeRunner, RunRequest, RunResult,
};
use crate::modules::execution::domain::entities::submission::SubmissionStatus;
use crate::modules::execution::infrastructure::repositories::submission_repository::SubmissionRepository;

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
    code_runner: C,
}

impl<C> ExecuteCodeCommandHandler<C>
where
    C: CodeRunner,
{
    pub fn new(submission_repository: SubmissionRepository, code_runner: C) -> Self {
        Self {
            submission_repository,
            code_runner,
        }
    }

    pub async fn handle(
        &self,
        command: ExecuteCodeCommand,
    ) -> Result<RunResult, Box<dyn std::error::Error>> {
        let request = RunRequest {
            language: command.language.clone(),
            code: command.code.clone(),
        };
        let result = self.code_runner.run(request).await?;
        let submission = self
            .submission_repository
            .create_submission(
                command.user_id,
                command.exercise_id,
                &command.code,
                &command.language,
            )
            .await?;

        let final_status = if result.exit_code == Some(124) {
            SubmissionStatus::Timeout
        } else if !result.success {
            SubmissionStatus::RuntimeError
        } else {
            SubmissionStatus::Passed
        };
        self.submission_repository
            .update_status(submission.id, final_status, Some(result.duration_ms as i32))
            .await?;

        Ok(result)
    }
}
