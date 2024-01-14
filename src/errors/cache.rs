use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
  #[error("Failed to initialize cache")]
  Initialize,

  #[error("Failed to manage cache directory")]
  FileSystemError(#[from] std::io::Error),
}