use async_trait::async_trait;

use crate::errors::ExecutionError;


#[async_trait]
pub trait Job {
  async fn run(&self) -> Result<(), ExecutionError>;
}