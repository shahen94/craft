use async_trait::async_trait;

use crate::{contracts::Pipe, errors::ExecutionError};

pub struct ExtractorPipe;

impl ExtractorPipe {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl Pipe<()> for ExtractorPipe {
  async fn run(&mut self) -> Result<(), ExecutionError> {
    Ok(())
  }
}