// ─── Folders ─────────────────────────────────────────────────────────────────

use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PACKAGES_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/packages");
    pub static ref REGISTRY_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/registry");
    pub static ref DEP_CACHE_FOLDER: PathBuf = PathBuf::from(".craft/cache/node_modules");
}

// ─── Files ───────────────────────────────────────────────────────────────────
