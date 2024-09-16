use std::process::{Command, Stdio};
use async_trait::async_trait;
use crate::actors::InstallActor;
use crate::contracts::Actor;

pub struct RunActor {
    pub script: String,
}

impl RunActor {
    pub fn new(script: String) -> Self {
        Self {
            script
        }
    }
}

#[async_trait]
impl Actor<crate::actors::install::PipeResult> for RunActor {
    async fn start(&mut self) -> crate::actors::install::PipeResult {
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", self.script.as_str()])
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