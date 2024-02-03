use std::{collections::HashMap, env, path::PathBuf};

use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::Package};

use super::constants::PACKAGES_CACHE_FOLDER;

#[derive(Debug)]
pub struct PackagesCache {
    pub directory: PathBuf,
    pub cache: Vec<String>,
}

impl PackagesCache {
    pub fn new() -> Self {
        let directory = {
            let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            home.push_str(PACKAGES_CACHE_FOLDER);

            PathBuf::from(home)
        };

        Self {
            directory,
            cache: Vec::new(),
        }
    }

    pub fn get_cache_directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn to_path_buf(&self, key: &str) -> PathBuf {
        self.directory.join(key)
    }
}

#[async_trait]
impl PersistentCache<PathBuf> for PackagesCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;

              println!("Packages cache directory created");
        }

        // We need to ls the directory and populate the cache

        let mut cache = Vec::new();

        let mut entries = tokio::fs::read_dir(&self.directory).await?;

        while let Ok(entry) = entries.next_entry().await {
            let entry = match entry {
                Some(entry) => entry,
                None => return Ok(()),
            };
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                cache.push(file_name);
            }
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

    async fn get(&self, key: &str) -> Option<PathBuf> {
        if self.has(key).await {
            return Some(self.to_path_buf(key));
        }

        None
    }
    async fn set(&mut self, key: &str, _: PathBuf) -> () {
        self.cache.push(key.to_string());
    }

    async fn has(&self, key: &str) -> bool {
        self.cache.contains(&key.to_owned())
    }
}
