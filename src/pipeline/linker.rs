use async_trait::async_trait;

use crate::{contracts::Pipe, errors::ExecutionError};

pub struct LinkerPipe;

impl LinkerPipe {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl Pipe<()> for LinkerPipe {
  async fn run(&mut self) -> Result<(), ExecutionError> {
    Ok(())
  }
}