use crate::actors::{ExecActor, PackageType, PreprocessDependencyInstall, RunActor};
use crate::command::{ConfigSubCommand, ProgramDesire};
use crate::contracts::{Logger, Pipe};
use crate::logger::CraftLogger;
use crate::pipeline::ConfigReader;
use crate::{
    actors::{CacheCleanActor, InstallActor},
    command::{Command, SubCommand},
    contracts::{Actor, Progress, ProgressAction},
    errors::ExecutionError,
    ui::UIProgress,
};
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
            SubCommand::Install(args_install) => {
                if args.is_install_without_args() {
                    let program_desire: ProgramDesire = args_install.into();
                    let deps_to_install = PreprocessDependencyInstall::new(program_desire)
                        .run()
                        .await?;

                    let err = InstallActor::new(deps_to_install).start().await;
                    if let Err(err) = err {
                        CraftLogger::error(format!("{}", err));
                    }

                    return Ok(());
                }

                let packages = args_install
                    .packages
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|p| {
                        if args_install.save_global {
                            PackageType::Global(p.to_string())
                        } else if args_install.save_prod {
                            PackageType::Prod(p.to_string())
                        } else if args_install.save_dev {
                            PackageType::Dev(p.to_string())
                        } else if args_install.save_optional {
                            PackageType::Optional(p.to_string())
                        } else {
                            PackageType::Prod(p.to_string())
                        }
                    })
                    .collect::<Vec<PackageType>>();

                InstallActor::new(packages).start().await?;

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
            SubCommand::Exec(e) => {
                CraftLogger::info(format!("Running command: {}", e.command));
                CraftLogger::info(format!("Args: {:?}", e.args));
                ExecActor::new(e.command, e.args).start().await?;

                Ok(())
            }
            SubCommand::Config(c) => {
                UIProgress::default();
                match c {
                    ConfigSubCommand::Set(s) => {
                        log::info!("{}", format!("Setting configuration: {:?}", s));
                        let mut conf = ConfigReader::new().run().await?;
                        conf.switch_global(s.global);
                        conf.switch_location(s.location);
                        conf.set_value(&s.key, Some(s.value))?;
                        Ok(())
                    }
                    ConfigSubCommand::Get(g) => {
                        let mut conf = ConfigReader::new().run().await?;
                        conf.get_value(g.key)?;
                        Ok(())
                    }
                    ConfigSubCommand::List(l) => {
                        let mut conf = ConfigReader::new().run().await?;
                        conf.switch_json(l.json);
                        conf.list_value()?;
                        Ok(())
                    }
                    _ => {
                        CraftLogger::info("Reading configuration".to_string());
                        Ok(())
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
