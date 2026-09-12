use tokio::process::Command;
use std::process::Stdio;


pub async fn podman_run() -> Result<String, Box<dyn std::error::Error>>{

    let mut cmd = Command::new("podman");

    cmd.args(&["run", "--rm", "alpine", "echo", "Hello World"]);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd.output().await?;

    if output.status.success(){
        let hasil = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(hasil)
    }else{
        let error = String::from_utf8_lossy(&output.stderr).into_owned();
                Err(format!("Podman error: {}", error).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_podman_run(){
        let result = podman_run().await;

        assert!(result.is_ok());


        let output = result.unwrap();
        assert_eq!(output.trim(), "Hello World");
    }
}
