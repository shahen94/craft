use crate::{
    cache::CacheManagerImpl,
    command::{CacheAction, Command, SubCommand},
    contracts::{Job, Logger},
    errors::ExecutionError,
    jobs::CacheJob,
    logger::CraftLogger,
    package::Package,
};

use crate::jobs::InstallJob;

pub struct Program {
    cache_manager: CacheManagerImpl,
}

impl Program {
    pub fn new() -> Self {
        Self {
            cache_manager: CacheManagerImpl::new(),
        }
    }
}

impl Program {
    pub async fn execute(&mut self, cmd: Command) -> Result<(), ExecutionError> {
        let logger = CraftLogger::new(cmd.verbose);

        logger.info(format!(
            "Craft Package Manager: {}",
            env!("CARGO_PKG_VERSION")
        ));

        if cmd.command.is_none() {
            logger.warn("No command provided");
            return Ok(());
        }

        let command = cmd.command.unwrap();

        match command {
            SubCommand::Install(action) => {
                logger.debug(format!("Installing package {}", &action.package));

                let package = Package::new(action.package).unwrap();

                InstallJob::new(package, logger).run().await.unwrap();
            }
            SubCommand::Cache(action) => match action {
                CacheAction::Clean => {
                    logger.info("Cleaning cache");
                    CacheJob::new(None).run().await?;
                }
            },
        }

        Ok(())
    }
}
