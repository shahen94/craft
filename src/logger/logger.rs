use colored::*;

use crate::common::contracts::Logger;

/// A logger that prints to stdout.
/// 
/// # Examples
/// 
/// ```
/// use craft::logger::CraftLogger;
/// use craft::common::contracts::Logger;
/// 
/// let logger = CraftLogger::new(true);
/// logger.log("Hello, world!");
/// logger.error("Hello, world!");
/// logger.warn("Hello, world!");
/// ```
pub struct CraftLogger {
  verbose: bool,
}

impl CraftLogger {
  pub fn new(verbose: bool) -> Self {
    Self { verbose }
  }
}

impl Logger for CraftLogger {
  fn log<S: AsRef<str>>(&self, message: S) {
    if !self.verbose {
      return;
    }
    println!("{}", message.as_ref().green());
  }

  fn error<S: AsRef<str>>(&self, message: S) {
    println!("{}", message.as_ref().red());
  }

  fn warn<S: AsRef<str>>(&self, message: S) {
    println!("{}", message.as_ref().yellow());
  }
}

