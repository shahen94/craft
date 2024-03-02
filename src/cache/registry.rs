use async_trait::async_trait;
use std::{collections::HashMap, env, fs::File, path::PathBuf};

use crate::{contracts::PersistentCache, errors::CacheError, package::NpmPackage};

use super::constants::{REGISTRY_CACHE_FILE, REGISTRY_CACHE_FOLDER};

// ─── RegistryCache ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct RegistryCache {
    pub directory: PathBuf,
    pub cache: HashMap<String, NpmPackage>,
}

// ───────────────────────────────────────────────────────────────────────────────

impl RegistryCache {
    pub async fn persist(&self) -> Result<(), CacheError> {
        let cache_file = File::create(self.directory.join(REGISTRY_CACHE_FILE)).unwrap();

        serde_json::to_writer(cache_file, &self.cache).unwrap();

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for RegistryCache {
    fn default() -> Self {
        let directory = {
            let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            home.push_str(REGISTRY_CACHE_FOLDER);

            PathBuf::from(home)
        };

        Self {
            directory,
            cache: HashMap::new(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl PersistentCache<NpmPackage> for RegistryCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(CacheError::FileSystemError)?;
            tokio::fs::File::create(self.directory.join(REGISTRY_CACHE_FILE))
                .await
                .map_err(CacheError::FileSystemError)?;
            return Ok(());
        }

        // load cache
        let stored_cache = match File::open(self.directory.join(REGISTRY_CACHE_FILE)) {
            Ok(file) => file,
            Err(_) => return Ok(()),
        };

        let stored_cache: HashMap<String, NpmPackage> =
            serde_json::from_reader(stored_cache).unwrap();

        self.cache = stored_cache;

        Ok(())
    }

    async fn clean(&self) -> Result<(), CacheError> {
        if self.directory.exists() {
            tokio::fs::remove_dir_all(&self.directory)
                .await
                .map_err(CacheError::FileSystemError)?;
        }

        Ok(())
    }

    async fn get(&self, key: &str) -> Option<NpmPackage> {
        self.cache.get(key).cloned()
    }

    async fn set(&mut self, key: &str, value: NpmPackage) -> () {
        self.cache.insert(key.to_string(), value);
    }

    async fn has(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}
