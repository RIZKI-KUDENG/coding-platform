use crate::modules::execution::application::ports::code_runner::{
    CodeRunner, RunRequest, RunResult,
};

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

    pub async fn handle(
        &self,
        command: RunCodeCommand,
    ) -> Result<RunResult, Box<dyn std::error::Error>> {
        let request = RunRequest {
            language: command.language.clone(),
            code: command.code.clone(),
        };
        let result = self.code_runner.run(request).await?;
        Ok(result)
    }
}
