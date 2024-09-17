use std::path::Path;
use crate::errors::LockfileError;
use crate::lockfile::lockfile_structure::LockfileStructure;

pub trait Lockfile<T> {
    fn read_lock_file(path: &Path) -> Result<T, LockfileError>;
    fn write_lock_file(path: &Path, lock: LockfileStructure) -> Result<T, LockfileError>;
}