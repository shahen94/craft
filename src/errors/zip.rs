use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZipError {
  #[error("Failed to open file")]
  FailedToOpen(#[from] std::io::Error),

  #[error("Failed to unzip file")]
  FailedToUnzip(String),
}