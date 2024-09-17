use std::fs;
use std::path::Path;
use crate::contracts::Lockfile;
use crate::errors::LockfileError;
use crate::lockfile::lockfile_structure::LockfileStructure;

pub struct LockFileActor;


impl Lockfile<LockfileStructure> for LockFileActor {
    fn read_lock_file(path: &Path) -> Result<LockfileStructure, LockfileError> {
        let file = fs::read_to_string(path).map_err(|e|LockfileError::FileReadError(e.to_string()))?;
        let structure = serde_yaml::from_str::<LockfileStructure>(&*file).map_err(|e|LockfileError::FileReadError(e.to_string()))?;
        Ok(structure)
    }

    fn write_lock_file(path: &Path, lock: LockfileStructure) -> Result<LockfileStructure, LockfileError> {
        let locked = serde_yaml::to_string(&lock).map_err(|e|LockfileError::InvalidStructure(e.to_string()))?;
        fs::write(path, locked).map_err(|e|LockfileError::FileWriteError(e.to_string()))?;
        Ok(lock)
    }
}