use crate::contracts::Actor;
use async_trait::async_trait;
use std::env;
use std::process::{Command, Stdio};

pub struct RunActor {
    pub script: String,
    pub cwd: Option<String>,
}

impl RunActor {
    pub fn new(script: String, cwd: Option<String>) -> Self {
        Self { script, cwd }
    }
}

#[async_trait]
impl Actor<crate::actors::install::PipeResult> for RunActor {
    async fn start(&mut self) -> crate::actors::install::PipeResult {
        let mut exec_path = env::current_dir().expect("Error getting cwd");

        if let Some(c) = self.cwd.clone() {
            exec_path = exec_path.join(c);
        }

        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", self.script.as_str()])
                .current_dir(exec_path)
                .stdout(Stdio::inherit())
                // creating a pipe to capture the standard error (stderr) of the child process.
                .stderr(Stdio::inherit())
                // output() executes the child process synchronously and captures its output.
                //It returns a std::process::Output struct containing information about the process's exit status, stdout, and stderr.
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .stdout(Stdio::inherit())
                .current_dir(exec_path)
                // creating a pipe to capture the standard error (stderr) of the child process.
                .stderr(Stdio::inherit())
                .arg(self.script.as_str())
                .spawn()
                .expect("failed to execute process")
        };
        let _ = child.wait().expect("child process wasn't running");

        Ok(())
    }
}
