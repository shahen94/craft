use std::sync::mpsc::Sender;

use async_trait::async_trait;

use crate::{
    contracts::{Phase, Pipe, ProgressAction},
    errors::ExecutionError,
};

#[derive(Debug)]
pub struct LinkerPipe {
    tx: Sender<ProgressAction>,
}

impl LinkerPipe {
    pub fn new(tx: Sender<ProgressAction>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl Pipe<()> for LinkerPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Linking));

        Ok(())
    }
}
