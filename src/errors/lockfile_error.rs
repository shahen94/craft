use thiserror::Error;

#[derive(Debug, Error)]
pub enum LockfileError {
    #[error("Error reading file {0}")]
    FileReadError(String),
    #[error("Error writing file {0}")]
    FileWriteError(String),
    #[error("Error file contains invalid structure {0}")]
    InvalidStructure(String),
}
