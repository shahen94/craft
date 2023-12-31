use colored::*;

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
  #[allow(dead_code)]
  verbose: bool,
}

impl CraftLogger {
  pub fn log<S: AsRef<str>>(message: S) {
    println!("{}", message.as_ref().green());
  }

  pub fn info<S: AsRef<str>>(message: S) {
    println!("{}", message.as_ref().blue());
  }

  pub fn error<S: AsRef<str>>(message: S) {
    println!("{}", message.as_ref().red());
  }

  pub fn warn<S: AsRef<str>>(message: S) {
    println!("{}", message.as_ref().yellow());
  }
}

