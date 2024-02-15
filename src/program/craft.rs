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

pub struct Program;

impl Program {
    pub fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
            let progress = UIProgress::default();

            progress.start(rx);
        })
    }

    fn read_package_json(&self) -> PackageJson {
        std::fs::read_to_string("package.json").unwrap().into()
    }

    pub async fn execute(&mut self, args: Command) -> Result<(), ExecutionError> {
        if args.command.is_none() {
            let json = self.read_package_json();

            let dependencies = json.dependencies;

            let mut packages = vec![];

            for (name, version) in dependencies {
                packages.push(format!("{}@{}", name, version));
            }

            InstallActor::new(packages).start().await.unwrap();

            return Ok(());
        }

        let command = args.command.unwrap();

        match command {
            SubCommand::Install(args) => {
                InstallActor::new(vec![args.package]).start().await.unwrap();
                Ok(())
            }
            SubCommand::Cache(args) => {
                CacheCleanActor::new(args).start().await;

                Ok(())
            }
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self
    }
}
