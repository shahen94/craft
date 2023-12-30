use clap::Parser;
use fs::Project;

mod cmd;
mod actors;
mod common;
mod logger;
mod fs;
mod registry;
mod cache;

use std::process;
use logger::CraftLogger;
use cmd::{Command, SubCommand, Install};
use common::{package::Package, contracts::{Logger, Actor}};

#[tokio::main]
async fn main() -> () {
  let data = Command::parse();
  let logger = CraftLogger::new(data.verbose);
  let actor = actors::InstallActions::new();
  let project = Project::new();

  if project.is_err() {
    logger.error(project.err().unwrap().reason);
    process::exit(1);
  }

  if data.command.is_none() {
    actor.install_all_packages().await;
    return;
  }

  let command = data.command.unwrap();

  match command {
    SubCommand::Install(Install { package }) => {
      let (name, version) = Package::parse_package(package);
      let package = match Package::new(name, version) {
        Ok(package) => package,
        Err(err) => {
          logger.error(err.reason);
          process::exit(1);
        }
      };

      actor.modules.init_folder();
      match actor.install_package(&package).await {
        Ok(_) => {}
        Err(err) => {
          logger.error(err.reason);
          process::exit(1);
        }
      };
    }
  }
}
