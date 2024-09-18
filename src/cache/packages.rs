use std::path::{Path, PathBuf};
use std::collections::HashSet;
use async_recursion::async_recursion;
use async_trait::async_trait;
use homedir::windows::my_home;
use crate::{contracts::PersistentCache, errors::CacheError};
use crate::cache::registry::convert_to_registry_key;
use crate::cache::RegistryKey;
use super::constants::PACKAGES_CACHE_FOLDER;

// ─── PackagesCache ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct PackagesCache {
    pub directory: PathBuf,
    pub cache: HashSet<RegistryKey>,
    pub downloaded_modules: HashSet<String>
}

// ───────────────────────────────────────────────────────────────────────────────

impl PackagesCache {
    #[async_recursion]
    pub async fn read_cache_directory(dir: &Path) -> Result<HashSet<RegistryKey>, CacheError> {
        let mut cache:HashSet<RegistryKey> = HashSet::new();

        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            /// Nested is e.g. @types/node etc.
            if entry.file_type().await?.is_dir() {
                let dirname = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| CacheError::CacheError)?;

                let p = dir.join(&dirname);
                let nested_cache = Self::read_cache_directory(&p).await?;

                for k in nested_cache {
                    let key = format!("{}/{}", dirname, k.to_string());
                    let key_with_subdir = convert_to_registry_key(&key);
                    cache.insert(key_with_subdir);
                }
            }
            let filename = entry
                .file_name()
                .into_string()
                .map_err(|_| CacheError::CacheError)?;
            let reg_key = convert_to_registry_key(&filename);

            cache.insert(reg_key);
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

    pub fn to_path_buf(&self, key: &str) -> PathBuf {
        self.directory.join(key)
    }

    pub fn diff_downloaded_modules(&self, modules: &Vec<String>) -> Vec<String> {
        let mut diff = vec![];

        for module in modules {
            if !self.downloaded_modules.contains(module) {
                diff.push(module.clone());
            }
        }

        diff
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for PackagesCache {
    fn default() -> Self {
        let directory = {
            

            my_home().unwrap().unwrap().join(PACKAGES_CACHE_FOLDER)
        };

        Self {
            directory,
            cache: HashSet::new(),
            downloaded_modules: HashSet::new()
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

    async fn get(&mut self, key: &RegistryKey) -> Option<PathBuf> {
        if self.has(key).await {
            return Some(self.directory.join::<PathBuf>(key.clone().into()));
        }

        None
    }
    async fn set(&mut self, key: &RegistryKey, _: PathBuf) -> () {
        self.cache.insert(key.clone());
    }

    async fn has(&mut self, key: &RegistryKey) -> bool {
        self.cache.contains(&key.to_owned())
    }
}
