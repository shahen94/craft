use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    contracts::{Job, Logger},
    errors::ExecutionError,
    logger::CraftLogger,
    package::{Package, RemotePackage},
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

impl InstallJob {
    pub async fn fetch_from_registry(&mut self) -> Result<RemotePackage, ExecutionError> {
        let package = self.package.clone();

        let logger = self.logger.clone();
        let npm_registry = self.npm_registry.clone();

        let result = tokio::spawn(async move {
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

            remote_package
        })
        .await;

        match result {
            Ok(remote_package) => Ok(remote_package),
            Err(e) => Err(ExecutionError::JobExecutionFailed(
                format!("Install({})", &self.package.name),
                e.to_string(),
            )),
        }
    }
}

#[async_trait]
impl Job for InstallJob {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let remote_package = self.fetch_from_registry().await?;

        let deps = remote_package
            .dependencies
            .iter()
            .chain(remote_package.dev_dependencies.iter());

        // Iterate over dependencies and devDependencies and install them
        for (name, version) in deps {
            let package_name = format!("{}@{}", name, version);
            let package = Package::new(&package_name);

            InstallJob::extend(package, self).run().await?;
        }

        Ok(())
    }
}
