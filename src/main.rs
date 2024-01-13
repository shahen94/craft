mod cmd;
mod common;
mod logger;
mod node;
mod registry;
mod cache;
mod executors;
mod program;

use clap::Parser;
use cmd::Command;
use program::Program;

#[tokio::main]
async fn main() -> () {
  let data = Command::parse();

  let mut program = Program::new(data).await;

  program.execute().await;
}
