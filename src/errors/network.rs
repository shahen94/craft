use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to download file")]
    FetchFailure(#[from] reqwest::Error),

    #[error("Failed to write file")]
    ErrorWhileWriting(#[from] tokio::io::Error),

    #[error("Failed to fetch version {0}")]
    FailedToFetchVersion(String),
    #[error("Checksum mismatch while downloading {0}")]
    CheckSum(String),
}
