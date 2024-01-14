use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    contracts::{Job, Logger},
    errors::ExecutionError,
    logger::CraftLogger,
    package::Package,
    registry::NpmRegistry,
};

pub struct InstallJob {
    package: Package,
    logger: Arc<Mutex<CraftLogger>>,
    npm_registry: Arc<Mutex<NpmRegistry>>,
}

impl InstallJob {
    pub fn new(package: Package, logger: CraftLogger) -> Self {
        Self {
            package,
            logger: Arc::new(Mutex::new(logger)),
            npm_registry: Arc::new(Mutex::new(NpmRegistry::new())),
        }
    }

    pub fn extend(package: Package, job: &InstallJob) -> Self {
        Self {
            package,
            logger: job.logger.clone(),
            npm_registry: job.npm_registry.clone(),
        }

    }
}

#[async_trait]
impl Job for InstallJob {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let package = self.package.clone();

        let logger = self.logger.clone();
        let mut npm_registry = self.npm_registry.clone();

        tokio::spawn(async move {
          let logger = logger.lock().await;
          let mut npm_registry = npm_registry.lock().await;

          let remote_package = npm_registry
            .get_package(package)
            .await
            .expect("Failed to fetch package");

          logger.debug(format!(
            "Fetched package: {}@{}",
            remote_package.name, remote_package.version
          ));
        }).await.unwrap();

        Ok(())
    }
}
