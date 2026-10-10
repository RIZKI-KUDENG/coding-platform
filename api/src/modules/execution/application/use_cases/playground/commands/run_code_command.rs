use crate::modules::execution::application::errors::ExecutionError;
use crate::modules::execution::application::ports::code_runner::{
    CodeRunner, RunRequest, RunResult,
};
use crate::modules::shared::error::InfrastructureError;

pub struct RunCodeCommand {
    pub language: String,
    pub code: String,
}

pub struct RunCodeCommandHandler<C>
where
    C: CodeRunner,
{
    code_runner: C,
}

impl<C> RunCodeCommandHandler<C>
where
    C: CodeRunner,
{
    pub fn new(code_runner: C) -> Self {
        Self { code_runner }
    }

    pub async fn handle(&self, command: RunCodeCommand) -> Result<RunResult, ExecutionError> {
        let request = RunRequest {
            language: command.language.clone(),
            code: command.code.clone(),
            input: None,
        };
        let result = self.code_runner.run(request).await.map_err(|e| {
            ExecutionError::RunnerUnavailable(InfrastructureError::new(e.to_string()))
        })?;
        Ok(result)
    }
}
