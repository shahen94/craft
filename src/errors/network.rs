use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
  #[error("Failed to download file")]
  FailedToFetch(#[from] reqwest::Error),

  #[error("Failed to write file")]
  FailedToWrite(#[from] tokio::io::Error),

  #[error("Failed to fetch version {0}")]
  FailedToFetchVersion(String),
}