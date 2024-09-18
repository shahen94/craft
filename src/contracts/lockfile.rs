use crate::errors::LockfileError;
use crate::lockfile::lockfile_structure::LockfileStructure;
use std::io;
use std::path::Path;

pub trait Lockfile<T> {
    fn read_lock_file(path: &Path) -> Result<T, LockfileError>;
    fn write_lock_file(path: &Path, lock: LockfileStructure) -> Result<T, LockfileError>;
    fn run(&self) -> Result<(), LockfileError>;
}
