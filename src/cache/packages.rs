use std::{collections::HashMap, env, path::PathBuf};

use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::Package};

use super::constants::PACKAGES_CACHE_FOLDER;

#[derive(Debug)]
pub struct PackagesCache {
    pub directory: PathBuf,
    pub cache: HashMap<String, Package>,
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
            cache: HashMap::new(),
        }
    }
}

#[async_trait]
impl PersistentCache<Package> for PackagesCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
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

    async fn get(&self, key: &str) -> Option<Package> {
        return None;
    }
    async fn set(&mut self, key: &str, value: Package) -> () {}
}
