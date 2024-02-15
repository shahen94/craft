use async_trait::async_trait;

use crate::{
    command::CacheAction,
    contracts::{Actor, Pipe},
    pipeline::CacheCleanPipe,
};

pub struct CacheCleanActor {
    action: CacheAction,
}

impl CacheCleanActor {
    pub fn new(action: CacheAction) -> Self {
        CacheCleanActor { action }
    }
}

#[async_trait]
impl Actor<()> for CacheCleanActor {
    async fn start(&mut self) -> () {
        let action = self.action.clone();
        let _ = CacheCleanPipe::new(action).run().await;
    }
}
