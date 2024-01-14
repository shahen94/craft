use clap::Parser;
use craft::command;
use craft::program;

#[tokio::main]
async fn main() -> () {
    let cmd = command::Command::parse();
    let mut craft = program::Program::new();

    craft.execute(cmd).await.unwrap();
}
