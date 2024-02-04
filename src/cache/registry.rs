use std::{collections::HashMap, env, fs::File, path::PathBuf};
use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::NpmPackage};

use super::constants::{REGISTRY_CACHE_FOLDER, REGISTRY_CACHE_FILE};

// ─── RegistryCache ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct RegistryCache {
    pub directory: PathBuf,
    pub cache: HashMap<String, NpmPackage>,
}

// ───────────────────────────────────────────────────────────────────────────────

impl RegistryCache {
    pub fn new() -> Self {
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

    pub async fn save(&self) -> Result<(), CacheError> {
        let cache_file = File::create(self.directory.join(REGISTRY_CACHE_FILE)).unwrap();

        serde_json::to_writer(cache_file, &self.cache).unwrap();

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl PersistentCache<NpmPackage> for RegistryCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
            tokio::fs::File::create(self.directory.join(REGISTRY_CACHE_FILE))
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
            return Ok(())
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
                .map_err(|err| CacheError::FileSystemError(err))?;
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
