use std::{collections::HashMap, path::PathBuf, str::FromStr};

use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::Package};

use super::constants::PACKAGES_CACHE_FOLDER;

#[derive(Debug)]
pub struct PackagesCache {
    pub cache: HashMap<String, Package>,
}

impl PackagesCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

#[async_trait]
impl PersistentCache<Package> for PackagesCache {
    async fn init(&self) -> Result<(), CacheError> {
        let cache_dir =
            PathBuf::from_str(PACKAGES_CACHE_FOLDER).map_err(|_| CacheError::Initialize)?;

        if !cache_dir.exists() {
            tokio::fs::create_dir_all(cache_dir)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
        }

        Ok(())
    }

    async fn clean(&self) -> Result<(), CacheError> {
        let cache_dir =
            PathBuf::from_str(PACKAGES_CACHE_FOLDER).map_err(|err| CacheError::Initialize)?;

        if cache_dir.exists() {
            tokio::fs::remove_dir_all(cache_dir)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
        }

        Ok(())
    }

    async fn get(&self, key: &str) -> Option<Package> {
        return None;
    }
    async fn set(&self, key: &str, value: Package) -> () {}
    
}
