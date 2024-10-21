use crate::contracts::Logger;

// ─── CraftLogger ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CraftLogger;

// ───────────────────────────────────────────────────────────────────────────────

impl CraftLogger {
    pub fn verbose<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::debug!("{}", message)
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl Logger for CraftLogger {
    fn info<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::info!("{}", message)
    }

    fn warn<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::warn!("{}", message)
    }

    fn error<S: AsRef<str> + std::fmt::Display>(message: S) {
        log::error!("{}", message)
    }
}
