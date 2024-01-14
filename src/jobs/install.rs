use async_trait::async_trait;

use crate::{package::Package, contracts::{Job, Logger}, errors::ExecutionError, logger::CraftLogger, cache::CacheManagerImpl};

pub struct InstallJob<T> where T: Logger {
  package: Package,
  logger: T,
}

impl InstallJob<CraftLogger> {
  pub fn new(package: Package, logger: CraftLogger) -> Self {
    Self { package, logger }
  }
}

#[async_trait]
impl Job for InstallJob<CraftLogger> {
  async fn run(&self) -> Result<(), ExecutionError> {
    todo!()
  }
}