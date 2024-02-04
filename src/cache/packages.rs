use std::{collections::HashMap, env, path::PathBuf};

use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError};

use super::constants::PACKAGES_CACHE_FOLDER;

// ─── PackagesCache ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct PackagesCache {
    pub directory: PathBuf,
    pub cache: HashMap<String, bool>,
}

// ───────────────────────────────────────────────────────────────────────────────

impl PackagesCache {
    pub fn new() -> Self {
        let directory = {
            let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            home.push_str(PACKAGES_CACHE_FOLDER);

            PathBuf::from(home)
        };

        Self {
            directory,
            cache: HashMap::new(),
        }
    }

    pub fn get_cache_directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn to_path_buf(&self, key: &str) -> PathBuf {
        self.directory.join(key)
    }
}

// ───────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl PersistentCache<PathBuf> for PackagesCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;

            return Ok(());
        }

        // We need to ls the directory and populate the cache

        let mut cache = HashMap::new();

        let mut entries = tokio::fs::read_dir(&self.directory).await?;

        while let Ok(entry) = entries.next_entry().await {
            let entry = match entry {
                Some(entry) => entry,
                None => break,
            };
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                cache.insert(file_name, true);
            }
        }

        self.cache = cache;

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

    async fn get(&self, key: &str) -> Option<PathBuf> {
        if self.has(key).await {
            return Some(self.to_path_buf(key));
        }

        None
    }
    async fn set(&mut self, key: &str, _: PathBuf) -> () {
        self.cache.insert(key.to_string(), true);
    }

    async fn has(&self, key: &str) -> bool {
        self.cache.contains_key(&key.to_owned())
    }
}
