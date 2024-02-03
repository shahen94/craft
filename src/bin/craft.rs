use clap::Parser;
use craft::command::Command;
use craft::program::Program;

#[tokio::main]
async fn main() -> () {
  let args = Command::parse();

  let mut program = Program::new();

  program.execute(args).await;
}
