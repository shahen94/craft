use async_trait::async_trait;

use crate::errors::ExecutionError;

#[async_trait]
pub trait Pipe<T> {
  async fn run(&mut self) -> Result<T, ExecutionError>;
}