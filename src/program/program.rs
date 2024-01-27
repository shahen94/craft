use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    cache::CacheManagerImpl,
    command::{CacheAction, Command, SubCommand},
    contracts::{CacheManager, Job, Logger},
    errors::ExecutionError,
    jobs::CacheJob,
    logger::CraftLogger,
    package::Package,
};

use crate::jobs::InstallJob;

pub struct Program {
    cache_manager: Arc<Mutex<CacheManagerImpl>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            cache_manager: Arc::new(Mutex::new(CacheManagerImpl::new())),
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

                let package = Package::new(&action.package);
                self.cache_manager
                  .lock()
                  .await
                  .init()
                  .await;

                InstallJob::new(package, logger)
                  .run()
                  .await
                  .unwrap();
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
