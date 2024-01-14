use async_trait::async_trait;

use crate::{contracts::{Job, CacheManager}, errors::ExecutionError, cache::CacheManagerImpl};

pub struct CacheJob {
  manager: CacheManagerImpl,
}

impl CacheJob {
  pub fn new(manager: Option<CacheManagerImpl>) -> Self {
    let manager = match manager {
      Some(manager) => manager,
      None => CacheManagerImpl::new(),
    };

    Self { manager }
  }
}

#[async_trait]
impl Job for CacheJob {
  async fn run(&self) -> Result<(), ExecutionError> {
    self.manager.clean().await;

    Ok(())
  }
}