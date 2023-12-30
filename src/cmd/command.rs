use clap::Parser;

/// Command line arguments
/// 
/// # Example
/// 
/// ```
/// let data = Command::parse();
/// println!("{:?}", data);
/// ```
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    pub command: Option<SubCommand>,

    #[arg(short, long)]
    #[clap(default_value = "false")]
    pub verbose: bool,
}

/// Sub commands
/// 
/// # Example
/// ```
/// let data = Command::parse();
/// let command = data.command.unwrap();
/// println!("{:?}", command);
/// ```
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(name = "install")]
    Install(Install),
}


/// Install sub command
///
/// # Example
/// ```
/// let data = Command::parse();
/// let command = data.command.unwrap();
/// let install = match command {
///  SubCommand::Install(install) => install,
/// _ => panic!("Invalid command")
/// };
#[derive(Debug, Parser)]
pub struct Install {
    #[clap(name = "package")]
    pub package: String,
}

