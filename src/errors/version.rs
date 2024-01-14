use thiserror::Error;

#[derive(Debug, Error)]
pub enum VersionError {
  #[error("Failed to fetch version {0} of package {1} from registry")]
  NotFound(String, String),

  #[error("Failed to parse version {0} of package {1}, reason: {2}")]
  Parse(String, String, String),
}