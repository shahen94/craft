use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{Level, LevelFilter};
use craft::command::Command;
use craft::program::Program;
use std::io::Write;




#[tokio::main]
async fn main() {
    let args = Command::parse();
    let mut program = Program;

    match program.execute(args).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}
