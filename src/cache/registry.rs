use std::{collections::HashMap, env, fs::File, path::PathBuf};
use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::RemotePackage};

use super::constants::REGISTRY_CACHE_FOLDER;

#[derive(Debug)]
pub struct RegistryCache {
    pub directory: PathBuf,
    pub cache: HashMap<String, RemotePackage>,
}

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

    async fn save(&self) -> Result<(), CacheError> {
        let cache_file = File::create(self.directory.join("cache.json")).unwrap();

        serde_json::to_writer(cache_file, &self.cache).unwrap();

        Ok(())
    }
}

#[async_trait]
impl PersistentCache<RemotePackage> for RegistryCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
        } else {
            // load cache
            let stored_cache = File::open(self.directory.join("cache.json")).unwrap();
            let stored_cache: HashMap<String, RemotePackage> =
                serde_json::from_reader(stored_cache).unwrap();

            self.cache = stored_cache;
        }

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

    async fn get(&self, key: &str) -> Option<RemotePackage> {
        self.cache.get(key).cloned()
    }

    async fn set(&mut self, key: &str, value: RemotePackage) -> () {
        self.cache.insert(key.to_string(), value);
        self.save().await.unwrap();
    }
}
