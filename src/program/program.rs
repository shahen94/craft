use crate::{
    command::{Command, SubCommand, CacheAction},
    contracts::{Job, Logger},
    errors::ExecutionError,
    logger::CraftLogger,
    package::Package, cache::CacheManager,
};

use crate::jobs::InstallJob;

pub struct Program;

impl Program {
    pub async fn execute(cmd: Command) -> Result<(), ExecutionError> {
        let logger = CraftLogger::new(cmd.verbose);
        logger.info(format!(
            "Craft Package Manager: {}",
            env!("CARGO_PKG_VERSION")
        ));

        if cmd.command.is_none() {
            logger.warn("No command provided");
            return Ok(());
        }

        match cmd.command.unwrap() {
            SubCommand::Install(action) => {
                logger.debug(format!("Installing package {}", &action.package));

                let package = Package::new(action.package).unwrap();

                InstallJob::new(package, logger).run().await.unwrap();
            }
            SubCommand::Cache(action) => {
              match action {
                CacheAction::Clean => {
                  logger.info("Cleaning cache");
                  let cache_manager = CacheManager::new();

                  cache_manager.clean().await;
                }
              }
            }
        }
        Ok(())
    }
}
