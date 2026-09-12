use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
}

#[allow(async_fn_in_trait)]
pub trait CodeRunner: Send + Sync {
    async fn run(&self, request: RunRequest) -> Result<RunResult, Box<dyn std::error::Error>>;
}
