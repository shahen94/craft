use clap::Parser;

/// Command line arguments
///
/// # Example
///
/// ```
/// use clap::Parser;
/// use craft::command::Command;
///
/// let data = Command::parse();
/// println!("{:?}", data);
/// ```
#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    pub command: SubCommand,
}

impl Command {
    pub fn is_install_without_args(&self) -> bool {
            if let SubCommand::Install(install) = self.command.clone() {
                return install.package.is_none()
        }
        false
    }
}


/// Sub commands
///
/// # Example
/// ```
/// use clap::Parser;
/// use craft::command::{Command, SubCommand};
///
/// let data = Command::parse();
/// let command = data.command;
/// println!("{:?}", command);
/// ```
#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[clap(name = "install", alias= "add")]
    Install(Install),
    #[clap(name = "run")]
    Run(Run),
    #[clap(name = "cache")]
    #[clap(subcommand)]
    Cache(CacheAction),
}

/// Install sub command
///
/// # Example
/// ```
/// use clap::Parser;
/// use craft::command::{Command, SubCommand, Install};
///
/// let data = Command::parse();
///
/// if data.is_install_without_args() {
///    println!("Reading package.json");
///     return;
/// }
/// let command = data.command;
/// let install = match command {
///  SubCommand::Install(install) => install,
/// _ => panic!("Invalid command")
/// };
#[derive(Debug, Parser, Clone)]
pub struct Install {
    #[clap(name = "package")]
    pub package: Option<String>,
    #[clap(name = "global", alias="g",value_delimiter = ' ', num_args = 1..)]
    pub global: Option<Vec<String>>,
    #[clap(name = "save-dev", alias="D",value_delimiter = ' ', num_args = 1..)]
    pub dev: Option<Vec<String>>,
    #[clap(name = "save-prod", alias="P",value_delimiter = ' ', num_args = 1..)]
    pub prod: Option<Vec<String>>
}


#[derive(Debug, Parser, Clone)]
pub struct Run {
    #[clap(name = "dir", alias="C")]
    pub directory: Option<String>,
    pub script: Option<String>,
}

#[derive(Debug, Parser, Clone)]
pub enum CacheAction {
    #[clap(name = "clean")]
    Clean,
}
