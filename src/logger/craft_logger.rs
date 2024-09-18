
use crate::contracts::Logger;

// ─── CraftLogger ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CraftLogger;

// ───────────────────────────────────────────────────────────────────────────────

impl CraftLogger {
    pub fn verbose<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::debug!("{}", message)
    }

    pub fn verbose_n<S: AsRef<str> + std::fmt::Display>(n: usize, message: S) {
        log::debug!("{}", message)
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl Logger for CraftLogger {
    fn log<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::info!("{}", message)
    }

    fn info<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::info!("{}", message)
    }

    fn error<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::error!("{}", message)
    }

    fn warn<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::warn!("{}", message)
    }
}
