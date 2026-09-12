use tokio::process::Command;
use std::process::Stdio;
use std::time::Instant;
use tokio::time::{timeout, Duration};

use crate::modules::execution::application::ports::code_runner::{CodeRunner, RunRequest, RunResult};


#[derive(Clone, Default)]
pub struct PodmanRunner;

impl PodmanRunner{
    pub fn new() -> Self{
        Self
    }
}

impl CodeRunner for PodmanRunner{
    async fn run(&self, request: RunRequest) -> Result<RunResult, Box<dyn std::error::Error>>{
        let start_time = Instant::now();

        let (image, runner_args) = match request.language.to_lowercase().as_str(){
            "python" => ("python:3.11-alpine", vec!["python", "-c", &request.code]),
            "javascript" => ("node:20-alpine", vec!["node", "-e", &request.code]),
            _ => return Err(format!("Unsupported language: {}", request.language).into())
        };

        let mut cmd = Command::new("podman");
        cmd.arg("run")
            .arg("--rm")
            .arg("--network=none")
            .arg("--memory=128m")
            .arg("--cpus=1")
            .arg(image)
            .args(&runner_args);

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let execution_timeout = Duration::from_secs(5);
        let output = match timeout(execution_timeout, cmd.output()).await{
            Ok(Ok(output)) => output,
            Ok(Err(err)) => return Err(format!("Failed to execute podman: {}", err).into()),
            Err(_) => {
                return Ok(RunResult{
                    success: false,
                    stdout: String::new(),
                    stderr: "execution timed out (exceeded 5 seconds).".to_string(),
                    exit_code: Some(124),
                    duration_ms: execution_timeout.as_millis() as u64
                });
            }
        };
        let duration_ms = start_time.elapsed().as_millis() as u64;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        Ok(RunResult{
            success: output.status.success(),
            stdout,
            stderr,
            exit_code: output.status.code(),
            duration_ms
        })
    }
}
