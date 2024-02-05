use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("Failed to execute job {0}: Reason: {1}")]
    JobExecutionFailed(String, String),
}
