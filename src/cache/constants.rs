// ─── Folders ─────────────────────────────────────────────────────────────────

use std::path::{PathBuf};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref  PACKAGES_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/packages");
pub static ref  REGISTRY_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/registry");
pub static ref  DEP_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/node_modules");
}


// ─── Files ───────────────────────────────────────────────────────────────────

