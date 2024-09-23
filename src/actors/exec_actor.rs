use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use async_trait::async_trait;
use crate::actors::install::PipeResult;
use crate::contracts::Actor;
use crate::errors::ExecutionError;

pub struct ExecActor {
    pub script: String,
    pub args: Option<Vec<String>>,
}

impl ExecActor {
    pub fn new(script: String, args: Option<Vec<String>>) -> Self {
        Self { script, args }
    }
}

pub enum ScriptType {
    Bash,
    Cmd,
    Pwsh,
}


impl ScriptType {
    pub fn get_script_ending(&self) -> String {
        match self {
            ScriptType::Bash => "".to_string(),
            ScriptType::Cmd => "cmd".to_string(),
            ScriptType::Pwsh => "ps1".to_string(),
        }
    }
}

fn get_file_ending_for_running() -> ScriptType {
    if cfg!(target_os = "windows") {
        if env::var("PSModulePath").is_ok() {
            ScriptType::Pwsh
        } else {
            ScriptType::Cmd
        }
    } else {
        ScriptType::Bash
    }
}

fn find_file_to_execute(script: &str, path_to_scan: &PathBuf) -> Option<(ScriptType, PathBuf)> {
    let file_ending = get_file_ending_for_running();

    let file_name_to_find = format!("{}.{}", script, file_ending.get_script_ending());

    let bin_dir = std::fs::read_dir(path_to_scan).unwrap();

    for entry in bin_dir {
        if let Ok(entry) = entry {
            let path_to_file = entry.path();
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                if file_name_to_find == file_name && entry.metadata().unwrap().is_file() {
                    return Some((file_ending, path_to_file));
                }
            }
        }
    }
    None
}


fn get_possible_script_paths() -> Vec<PathBuf> {
    let mut paths = vec![];

    let bin_path = env::current_dir().unwrap().join("node_modules").join("\
        .bin");
    let bin_dir = std::fs::metadata(&bin_path);

    if bin_dir.is_ok() {
        paths.push(bin_path);
    }

    paths
}

#[async_trait]
impl Actor<PipeResult> for ExecActor {
    async fn start(&mut self) -> PipeResult {
        let exec_path = env::current_dir().expect("Error getting cwd");

        let mut possible_scripts = vec![];

        get_possible_script_paths().iter().for_each(|path| {
            if let Some(p) = find_file_to_execute(self.script.as_str(), path) {
                possible_scripts.push(p);
            }
        });

        if possible_scripts.is_empty() {
            return Err(ExecutionError::NoScriptsFound);
        }

        let mut command_to_execute;

        let (shell, script) = &possible_scripts[0];
        match shell {
            ScriptType::Bash => {
                command_to_execute = Command::new("sh");
                command_to_execute.arg("-c")
                    .stdout(Stdio::inherit())
                    .current_dir(exec_path)
                    .stderr(Stdio::inherit())
                    .arg(script);
            }
            ScriptType::Cmd => {
                command_to_execute =  Command::new("cmd");
                command_to_execute.args(["/C", script.to_str().unwrap()])
                    .current_dir(exec_path)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());
            }
            ScriptType::Pwsh => {
                command_to_execute = Command::new("pwsh");
                command_to_execute.args(["-File", script.to_str().unwrap()])
                    .current_dir(exec_path)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());
            }
        };

        if let Some(args) = &self.args {
            command_to_execute.args(args);
        }

        let mut child = command_to_execute.spawn().expect("failed to execute process");


        let _ = child.wait().expect("child process wasn't running");

        Ok(())
    }
}