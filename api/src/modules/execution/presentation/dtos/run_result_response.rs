use serde::Serialize;

use crate::modules::execution::application::ports::code_runner::RunResult;

#[derive(Debug, Serialize)]
pub struct RunResultResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
}

impl From<RunResult> for RunResultResponse {
    fn from(result: RunResult) -> Self {
        Self {
            success: result.success,
            stdout: result.stdout,
            stderr: result.stderr,
            exit_code: result.exit_code,
            duration_ms: result.duration_ms,
        }
    }
}
