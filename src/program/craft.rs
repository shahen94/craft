use crate::actors::{PreprocessDependencyInstall, RunActor};
use crate::command::ProgramDesire;
use crate::contracts::Logger;
use crate::logger::CraftLogger;
use crate::{
    actors::{CacheCleanActor, InstallActor},
    command::{Command, SubCommand},
    contracts::{Actor, Progress, ProgressAction},
    errors::ExecutionError,
    package::PackageJson,
    ui::UIProgress,
};
use std::collections::HashMap;
use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

pub struct Program;

impl Program {
    pub fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
            let progress = UIProgress::default();

            progress.start(rx);
        })
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        let command = args.command.clone();

        match command {
            SubCommand::Install(argsInstall) => {
                if args.is_install_without_args() {
                    let program_desire: ProgramDesire = argsInstall.into();
                    let deps_to_install = PreprocessDependencyInstall::new(program_desire)
                        .run()
                        .await
                        .unwrap();

                    InstallActor::new(deps_to_install).start().await.unwrap();
                } else if argsInstall.save_optional {
                    let packages = argsInstall.packages.clone().unwrap();
                    InstallActor::new(packages).start().await.unwrap();
                } else if argsInstall.save_dev {
                    let packages = argsInstall.packages.clone().unwrap();
                    InstallActor::new(packages).start().await.unwrap();
                } else if argsInstall.save_prod {
                    let packages = argsInstall.packages.clone().unwrap();
                    InstallActor::new(packages).start().await.unwrap();
                } else if argsInstall.global {
                    let packages = argsInstall.packages.clone().unwrap();
                    InstallActor::new(packages).start().await.unwrap();
                }

                Ok(())
            }
            SubCommand::Cache(args) => {
                CacheCleanActor::new(args).start().await;

                Ok(())
            }
            SubCommand::Run(r) => {
                let json = PreprocessDependencyInstall::get_script()?;

                if json.is_empty() {
                    return Err(ExecutionError::JobExecutionFailed(
                        "script must be exactly 1".to_string(),
                        "script must be exactly 1".to_string(),
                    ));
                }

                if let Some(script) = json.get(&r.script) {
                    CraftLogger::info(format!("Running script: {}", script));
                    CraftLogger::info(format!("Command: {}", script));
                    RunActor::new(script.clone(), r.directory).start().await?;

                    Ok(())
                } else {
                    CraftLogger::error(format!("Script {} not found", r.script));
                    Err(ExecutionError::ScriptNotFound(format!(
                        "Script {} not found",
                        r.script
                    )))
                }
            }
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self
    }
}
