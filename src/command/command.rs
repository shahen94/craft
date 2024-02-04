use clap::Parser;

/// Command line arguments
/// 
/// # Example
/// 
/// ```
/// let data = Command::parse();
/// println!("{:?}", data);
/// ```
#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    pub command: Option<SubCommand>,
}

/// Sub commands
/// 
/// # Example
/// ```
/// let data = Command::parse();
/// let command = data.command.unwrap();
/// println!("{:?}", command);
/// ```
#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[clap(name = "install")]
    Install(Install),

    #[clap(name = "cache")]
    #[clap(subcommand)]
    Cache(CacheAction),
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
#[derive(Debug, Parser, Clone)]
pub struct Install {
    #[clap(name = "package")]
    pub package: String,
}

#[derive(Debug, Parser, Clone)]
pub enum CacheAction {
    #[clap(name = "clean")]
    Clean,
}
