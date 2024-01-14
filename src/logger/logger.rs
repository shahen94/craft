use colored::Colorize;

use crate::contracts::Logger;

#[derive(Debug, Clone)]
pub struct CraftLogger {
    verbose: bool,
}

impl CraftLogger {
    pub fn new(verbose: bool) -> Self {
        CraftLogger { verbose }
    }
}

impl Logger for CraftLogger {
    fn log<S: AsRef<str>>(&self, message: S) {
        println!("{}", message.as_ref().green());
    }

    fn info<S: AsRef<str>>(&self, message: S) {
        println!("{}", message.as_ref().blue());
    }

    fn debug<S: AsRef<str>>(&self, message: S) {
        if self.verbose {
            println!("{}", message.as_ref().bold().purple());
        }
    }

    fn error<S: AsRef<str>>(&self, message: S) {
        println!("{}", message.as_ref().red());
    }

    fn warn<S: AsRef<str>>(&self, message: S) {
        println!("{}", message.as_ref().yellow());
    }
}
