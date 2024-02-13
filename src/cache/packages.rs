use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use async_recursion::async_recursion;
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
    #[async_recursion]
    pub async fn read_cache_directory(dir: &Path) -> Result<HashMap<String, bool>, CacheError> {
        let mut cache = HashMap::new();

        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let dirname = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CacheError::CacheError)?;

                let p = dir.join(&dirname);
                let nested_cache = Self::read_cache_directory(&p).await?;

                for (k, v) in nested_cache {
                    let key = format!("{}/{}", dirname, k);
                    cache.insert(key, v);
                }
            }

            cache.insert(
                entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CacheError::CacheError)?,
                true,
            );
        }

        Ok(cache)
    }

    pub fn get_cache_directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn to_path_buf(&self, key: &str) -> PathBuf {
        self.directory.join(key)
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for PackagesCache {
    fn default() -> Self {
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
}

// ───────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl PersistentCache<PathBuf> for PackagesCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(CacheError::FileSystemError)?;

            return Ok(());
        }

        self.cache = Self::read_cache_directory(&self.directory).await?;

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
