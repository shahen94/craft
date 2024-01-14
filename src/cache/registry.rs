use std::{collections::HashMap, path::PathBuf, str::FromStr};

use async_trait::async_trait;

use crate::{contracts::PersistentCache, errors::CacheError, package::Package};

use super::constants::REGISTRY_CACHE_FOLDER;

#[derive(Debug)]
pub struct RegistryCache {
    pub cache: HashMap<String, Package>,
}

impl RegistryCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

#[async_trait]
impl PersistentCache<Package> for RegistryCache {
    async fn init(&self) -> Result<(), CacheError> {
        let cache_dir =
            PathBuf::from_str(REGISTRY_CACHE_FOLDER).map_err(|err| CacheError::Initialize)?;

        if !cache_dir.exists() {
            tokio::fs::create_dir_all(cache_dir)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
        }

        Ok(())
    }

    async fn clean(&self) -> Result<(), CacheError> {
        let cache_dir =
            PathBuf::from_str(REGISTRY_CACHE_FOLDER).map_err(|err| CacheError::Initialize)?;

        if cache_dir.exists() {
            tokio::fs::remove_dir_all(cache_dir)
                .await
                .map_err(|err| CacheError::FileSystemError(err))?;
        }

        Ok(())
    }

    async fn get(&self, key: &str) -> Option<Package> {
        todo!()
    }

    async fn set(&self, key: &str, value: Package) -> () {}
}
