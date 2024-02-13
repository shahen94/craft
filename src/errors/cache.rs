use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Failed to manage cache directory")]
    FileSystemError(#[from] std::io::Error),

    #[error("Failed to manage cache directory")]
    CacheError,
}
