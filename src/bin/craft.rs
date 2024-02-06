use clap::Parser;
use craft::command::Command;
use craft::program::Program;

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
