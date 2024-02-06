use async_trait::async_trait;

use crate::{
    cache::{PackagesCache, RegistryCache},
    command::CacheAction,
    contracts::{PersistentCache, Pipe},
    errors::ExecutionError,
};

pub struct CacheCleanPipe {
    action: CacheAction,
}

impl CacheCleanPipe {
    pub fn new(action: CacheAction) -> Self {
        Self { action }
    }
}

// ─── Implementations ─────────────────────────────────────────────────────────

#[async_trait]
impl Pipe<()> for CacheCleanPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        match self.action {
            CacheAction::Clean => {
                let _ = PackagesCache::default().clean().await;
                let _ = RegistryCache::default().clean().await;
            }
        }

        Ok(())
    }
}
