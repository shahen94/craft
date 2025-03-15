use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("Failed to execute job {0}: Reason: {1}")]
    JobExecutionFailed(String, String),
    #[error("Failed to find pacakge.json in current working directory")]
    PackageJsonNotFound,
    #[error("Failed to find script {0}")]
    ScriptNotFound(String),
    #[error("Failed to find a script in node_modules/.bin")]
    NoScriptsFound,
    #[error("Failed to parse .npmrc file")]
    ConfigError(String),
}
