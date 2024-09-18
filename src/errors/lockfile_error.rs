
//#[derive(Debug, Error)]
pub enum LockfileError {
    //#[error("Error reading file {0}")]
    FileReadError(String),
    FileWriteError(String),
    InvalidStructure(String)
}