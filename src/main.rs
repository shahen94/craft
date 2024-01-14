use clap::Parser;

mod command;
mod package;
mod cache;
mod contracts;
mod errors;
mod registry;
mod program;
mod logger;
mod network;
mod tar;
mod jobs;

#[tokio::main]
async fn main() -> () {
    let cmd = command::Command::parse();

    program::Program::execute(cmd).await.unwrap();
}
