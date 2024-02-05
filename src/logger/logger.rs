use colored::Colorize;

use crate::contracts::{Logger, CRAFT_VERBOSE_LOGGING};

// ─── CraftLogger ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CraftLogger;

// ───────────────────────────────────────────────────────────────────────────────

impl CraftLogger {
    pub fn is_verbose() -> bool {
        let verbose = std::env::var(CRAFT_VERBOSE_LOGGING).unwrap_or("false".to_string());
        let verbose = verbose.parse::<bool>().unwrap_or(false);

        verbose
    }

    pub fn verbose<S: AsRef<str>>(message: S) {
        if CraftLogger::is_verbose() {
            let prefix = "[VERBOSE]:".bold().red();
            println!("{} {}", prefix, message.as_ref().bold().purple());
        }
    }

    pub fn verbose_n<S: AsRef<str>>(n: usize, message: S) {
        if CraftLogger::is_verbose() {
            let newline = "\n".repeat(n);
            let prefix = "[VERBOSE]:".bold().red();
            println!("{}{} {}", newline, prefix, message.as_ref().bold().purple());
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl Logger for CraftLogger {
    fn log<S: AsRef<str>>(message: S) {
        println!("{}", message.as_ref().green());
    }

    fn info<S: AsRef<str>>(message: S) {
        println!("{}", message.as_ref().blue());
    }

    fn error<S: AsRef<str>>(message: S) {
        println!("{}", message.as_ref().red());
    }

    fn warn<S: AsRef<str>>(message: S) {
        println!("{}", message.as_ref().yellow());
    }
}
