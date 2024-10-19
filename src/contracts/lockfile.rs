use crate::errors::LockfileError;
use std::path::Path;

pub trait Lockfile<T> {
    fn read_lock_file(path: &Path) -> Result<T, LockfileError>;
    fn run(&self) -> Result<(), LockfileError>;
}
