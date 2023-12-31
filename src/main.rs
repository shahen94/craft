use clap::Parser;

mod cmd;

mod common;
mod logger;
mod fs;
mod registry;
mod cache;
mod executors;

use std::process;
use logger::CraftLogger;
use cmd::{Command, SubCommand, Install, CacheAction};
use common::{package::Package, contracts::Actor};

#[tokio::main]
async fn main() -> () {
  let data = Command::parse();
  let mut actor = executors::InstallActions::new(None);

  actor.modules.init_folder();

  if data.command.is_none() {
    actor.install_all_packages().await;
    return
  }

  let command = data.command.unwrap();

  match command {
    SubCommand::Cache(CacheAction::Clean) => {
      actor.clean_cache().await;
      CraftLogger::log("Cache cleaned");
    }

    SubCommand::Install(Install { package }) => {
      let (name, version) = Package::parse_package(package);
      let package = match Package::new(name, version) {
        Ok(package) => package,
        Err(err) => {
          CraftLogger::error(err.reason);
          process::exit(1);
        }
      };

      match actor.install_package(&package).await {
        Ok(_) => {}
        Err(err) => {
          CraftLogger::error(err.reason);
          process::exit(1);
        }
      };
    }
  }
}
