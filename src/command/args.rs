use std::{env, fs};
use clap::Parser;
/// Command line arguments
///
/// # Example
///
/// ```no_run
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
            return install.packages.is_none();
        }
        false
    }
}


impl Into<ProgramDesire> for Install {
    fn into(self) -> ProgramDesire {
        let node_env = env::var("NODE_ENV").unwrap_or("development".to_string());
        let mut program_desire = ProgramDesire {
            dev_install: true,
            prod_install: true,
            optional_install: true,
            global_install: false,
            package_json_available: false,
            pnpm_lock_yaml_available: false,
        };

        // This needs to be done before all the other checks
        program_desire.package_json_available = fs::exists("package.json").unwrap_or(false);
        program_desire.pnpm_lock_yaml_available = fs::exists("pnpm-lock.yaml").unwrap_or(false);

        // In that case we only install dev dependencies
        if self.dev {
            program_desire.prod_install = false;
            program_desire.optional_install = false;
            return program_desire;
        }

        if self.save_prod {
            program_desire.dev_install = false;
            program_desire.optional_install = false;
            return program_desire;
        }




        /// If no optional we don't install optional dependencies
        if self.no_optional {
            program_desire.optional_install = false;
        }

        if self.global {
            program_desire.global_install = true;
            program_desire.dev_install = false;
            program_desire.prod_install = false;
            program_desire.optional_install = false;
        }

        if node_env == "production" {
            program_desire.dev_install = false;
            program_desire.optional_install = false;
        }




        program_desire
    }
}

/// Determines what should be installed. This is additive so
/// -> dev_install => dev, prod
/// -> prod_install => prod
/// -> optional_install => optional, dev, prod
/// -> global_install => global
///
pub struct ProgramDesire {
    pub dev_install: bool,
    pub prod_install: bool,
    pub optional_install: bool,
    pub global_install: bool,
    pub package_json_available: bool,
    pub pnpm_lock_yaml_available: bool,
}


/// Sub commands
///
/// # Example
/// ```no_run
/// use clap::Parser;
/// use craft::command::{Command, SubCommand};
///
/// let data = Command::parse();
/// let command = data.command;
/// println!("{:?}", command);
/// ```
#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    #[clap(name = "install", alias = "add")]
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
/// ```no_run
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

    #[arg(long)]
    pub offline: bool,
    #[arg(long)]
    pub prod: bool,
    #[arg(long)]
    pub dev: bool,
    #[arg(long)]
    pub no_optional: bool,
    #[arg(long)]
    pub no_peers: bool,
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
    #[clap(name = "dir", alias = "C", required = false, index = 2)]
    pub directory: Option<String>,
    #[arg(required = true, name = "--script", index = 1)]
    pub script: String,
}

#[derive(Debug, Parser, Clone)]
pub enum CacheAction {
    #[clap(name = "clean")]
    Clean,
}
