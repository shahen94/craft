use super::constants::PACKAGES_CACHE_FOLDER;
use crate::cache::registry::convert_to_registry_key;
use crate::cache::RegistryKey;
use crate::fs::get_config_dir;
use crate::{contracts::PersistentCache, errors::CacheError};
use async_recursion::async_recursion;
use async_trait::async_trait;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
// ─── PackagesCache ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PackagesCache {
    pub directory: PathBuf,
    pub cache: HashSet<RegistryKey>,
    pub downloaded_modules: HashSet<String>,
}

// ───────────────────────────────────────────────────────────────────────────────

impl PackagesCache {
    #[async_recursion]
    pub async fn read_cache_directory(dir: &Path) -> Result<HashSet<RegistryKey>, CacheError> {
        let mut cache: HashSet<RegistryKey> = HashSet::new();

        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            // Nested is e.g. @types/node etc.
            if entry.file_type().await?.is_dir() {
                let dirname = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CacheError::CacheError)?;

                let p = dir.join(&dirname);
                let nested_cache = Self::read_cache_directory(&p).await?;

                for k in nested_cache {
                    let key = format!("{}/{}", dirname, k);
                    let key_with_subdir = convert_to_registry_key(&key);
                    cache.insert(key_with_subdir);
                }
            } else {
                let filename = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CacheError::CacheError)?;
                let reg_key = convert_to_registry_key(&filename);

                cache.insert(reg_key);
            }
        }

        Ok(cache)
    }

    pub async fn read_node_modules(dir: &Path) -> Result<HashSet<String>, CacheError> {
        let mut entries = tokio::fs::read_dir(dir).await?;
        let mut cache = HashSet::new();

        while let Some(entry) = entries.next_entry().await? {
            let file_name = entry.file_name().to_os_string().into_string().unwrap();
            cache.insert(file_name);
        }

        Ok(cache)
    }

    pub fn get_cache_directory(&self) -> &PathBuf {
        &self.directory
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for PackagesCache {
    fn default() -> Self {
        let directory = { get_config_dir(PACKAGES_CACHE_FOLDER.clone()) };

        Self {
            directory,
            cache: HashSet::new(),
            downloaded_modules: HashSet::new(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl PersistentCache<PathBuf> for PackagesCache {
    async fn init(&mut self) -> Result<(), CacheError> {

        self.cache = Self::read_cache_directory(&self.directory).await?;
        self.downloaded_modules = Self::read_node_modules(&self.directory).await?;

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

    async fn has(&mut self, key: &RegistryKey) -> bool {
        self.cache.contains(&key.to_owned())
    }
    async fn get(&mut self, key: &RegistryKey) -> Option<PathBuf> {
        if self.has(key).await {
            return Some(self.directory.join::<PathBuf>(key.clone().into()));
        }

        None
    }

    async fn set(&mut self, key: &RegistryKey, _: PathBuf) -> () {
        self.cache.insert(key.clone());
    }
}
