use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use crate::{
    actors::{CacheCleanActor, InstallActor},
    command::{Command, SubCommand},
    contracts::{Actor, Progress, ProgressAction},
    errors::ExecutionError,
    package::PackageJson,
    ui::UIProgress,
};
use crate::actors::RunActor;
use crate::contracts::Logger;
use crate::logger::CraftLogger;

pub struct Program;

impl Program {
    pub fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
            let progress = UIProgress::default();

            progress.start(rx);
        })
    }

    fn read_package_json(&self) -> Result<PackageJson, ExecutionError> {
        std::fs::read_to_string("package.json").map(|e|e.into()).map_err(|_|
            ExecutionError::PackageJsonNotFound)
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        if args.is_install_without_args() {
            let json = self.read_package_json()?;

            let dependencies = json.dependencies;

            let mut packages = vec![];

            for (name, version) in dependencies {
                packages.push(format!("{}@{}", name, version));
            }

            InstallActor::new(packages).start().await.unwrap();

            return Ok(());
        }

        let command = args.command;

        match command {
            SubCommand::Install(args) => {
                let packages = args.packages.unwrap();
                if args.save_optional {
                    InstallActor::new(packages).start().await.unwrap();
                } else if args.save_dev {
                    InstallActor::new(packages).start().await.unwrap();
                } else if args.save_prod {
                    InstallActor::new(packages).start().await.unwrap();
                } else if args.global {
                    InstallActor::new(packages).start().await.unwrap();
                } else {
                    InstallActor::new(packages).start().await.unwrap();
                }

                Ok(())
            }
            SubCommand::Cache(args) => {
                CacheCleanActor::new(args).start().await;

                Ok(())
            }
            SubCommand::Run(r)=>{
                let json = self.read_package_json()?;

                if r.script.is_none() {
                    return Err(ExecutionError::JobExecutionFailed("script must be exactly 1".to_string(), "script must be exactly 1".to_string()))
                }
                let script = r.script.unwrap();

                match json.scripts {
                    Some(scripts)=> {
                        if let Some(script) = scripts.get(&script) {
                            CraftLogger::info(format!("Running script: {}", script));
                            CraftLogger::info(format!("Command: {}", script));

                            RunActor::new(script.clone(), r.directory).start().await?;


                            Ok(())
                        } else {
                            CraftLogger::error(format!("Script {} not found", script));
                            Err(ExecutionError::ScriptNotFound(format!("Script {} not found", script)))
                        }
                    }
                    None => {
                        CraftLogger::error("No scripts found in package.json");
                        Err(ExecutionError::NoScriptsFound)
                    }
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
