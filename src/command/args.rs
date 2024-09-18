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
                return  install.packages.is_none();
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
    #[arg(name = "global", long, short, alias = "g")]
    pub global: bool,
    /// Save as dev dependency
    #[arg(long)]
    pub save_dev: bool,

    /// Save as production dependency
    #[arg(long)]
    pub save_prod: bool,

    /// Save as optional dependency
    #[arg(long)]
    pub save_optional: bool,

    /// List of packages to install
    #[arg(required = false)]
    pub packages: Option<Vec<String>>,
}


#[derive(Debug, Parser, Clone)]
pub struct Run {
    #[clap(name = "dir", alias="C", required=false, index=2)]
    pub directory: Option<String>,
    #[arg(required = true, name="--script", index=1)]
    pub script: String,
}

#[derive(Debug, Parser, Clone)]
pub enum CacheAction {
    #[clap(name = "clean")]
    Clean,
}
