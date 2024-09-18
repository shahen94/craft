use crate::{contracts::PersistentCache, errors::CacheError, package::NpmPackage};
use async_trait::async_trait;
use homedir::my_home;
use nodejs_semver::{Range, Version};
use std::fmt::Display;
use std::{collections::HashMap, fs::File, io, path::PathBuf};

use super::constants::REGISTRY_CACHE_FOLDER;

//
#[derive(Eq, Debug, Hash, PartialEq, Clone)]
pub struct RegistryKey {
    pub name: String,
    pub version: String,
}

impl Display for RegistryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

impl From<RegistryKey> for PathBuf {
    fn from(val: RegistryKey) -> Self {
        PathBuf::from(format!("{}@{}", val.name, val.version))
    }
}

impl From<&NpmPackage> for RegistryKey {
    fn from(pkg: &NpmPackage) -> Self {
        RegistryKey {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
        }
    }
}

pub fn convert_to_registry_key(key: &str) -> RegistryKey {
    if key.starts_with("@") {
        let key_version_arr = key.split("@").collect::<Vec<&str>>();
        let name = format!("@{}{}", key_version_arr[0], key_version_arr[1]);

        if key_version_arr.len() == 2 {
            log::info!("Key: {}", key);
        }

        return RegistryKey {
            name,
            version: key_version_arr[2].to_string(),
        };
    }

    let key_version_arr = key.split("@").collect::<Vec<&str>>();
    RegistryKey {
        name: key_version_arr[0].to_string(),
        version: key_version_arr[1].to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_to_registry_key() {
        let key = "express@4.17.1";
        let registry_key = super::convert_to_registry_key(key);
        assert_eq!(registry_key.name, "express");
        assert_eq!(registry_key.version, "4.17.1");
    }

    #[test]
    fn test_convert_scoped_package() {
        let key = "@types/node@14.14.37";
        let registry_key = super::convert_to_registry_key(key);
        assert_eq!(registry_key.name, "@types/node");
        assert_eq!(registry_key.version, "14.14.37");
    }
}

// ─── RegistryCache ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct RegistryCache {
    pub directory: PathBuf,
    // express -> 4.17.1 -> NpmPackage
    pub cache: HashMap<String, HashMap<String, NpmPackage>>,
}

// ───────────────────────────────────────────────────────────────────────────────

impl RegistryCache {
    /// Update to a given versions also updates the complete file
    pub async fn persist(&self, key: &RegistryKey) -> Result<(), CacheError> {
        let path_to_use: PathBuf;

        // @types/node -> Should go to a separate folder @types and contain a file node
        if key.name.contains("/") {
            let splitted_val = key.name.split("/").collect::<Vec<&str>>();
            let dir_to_create = self.directory.join(splitted_val[0]);
            if !dir_to_create.exists() {
                tokio::fs::create_dir_all(&dir_to_create).await?;
            }

            path_to_use = dir_to_create.join(format!("{}.json", splitted_val[1]));
        } else {
            path_to_use = self.directory.join(format!("{}.json", key.name));
        }

        let cache_file = File::create(path_to_use);

        if cache_file.is_err() {
            println!("ERror for key {}", key)
        }

        let key_to_save = self.cache.get(&key.name).unwrap();
        serde_json::to_writer(cache_file.unwrap(), key_to_save).unwrap();

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for RegistryCache {
    fn default() -> Self {
        let directory = {
            my_home()
                .unwrap()
                .unwrap()
                .join(REGISTRY_CACHE_FOLDER.clone())
        };

        Self {
            directory,
            cache: HashMap::new(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl RegistryCache {
    fn load_file(&self, key: &RegistryKey) -> Result<HashMap<String, NpmPackage>, io::Error> {
        // Not yet loaded into cache
        let cache_file = File::open(self.directory.join(format!("{}.json", key.name))).unwrap();

        // Loads complete configuration
        let cache: HashMap<String, NpmPackage> = serde_json::from_reader(cache_file)?;
        Ok(cache)
    }

    fn perform_preload(&mut self, key: &RegistryKey) {
        if let Some(cache_key) = self.cache.get(&key.name) {
            // We already have information about the package
            if cache_key.is_empty() {
                let loaded_key = self.load_file(key);
                match loaded_key {
                    Ok(loaded_key) => {
                        self.cache.insert(key.name.clone(), loaded_key);
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to load cache file for {} with {}.",
                            key.name,
                            e.to_string()
                        );
                    }
                }
            }
        }
    }
}

#[async_trait]
impl PersistentCache<NpmPackage> for RegistryCache {
    async fn init(&mut self) -> Result<(), CacheError> {
        if !self.directory.exists() {
            tokio::fs::create_dir_all(&self.directory)
                .await
                .map_err(CacheError::FileSystemError)?;
            return Ok(());
        }

        // load cache
        let mut entries = tokio::fs::read_dir(&self.directory).await?;

        while let Some(entry) = entries.next_entry().await? {
            let file_or_dir_name = entry.file_name().to_string_lossy().to_string();
            // There is hopefully only one / in the name @babel/core
            if entry.file_type().await?.is_dir() {
                // We have a directory
                let mut sub_entries = tokio::fs::read_dir(entry.path()).await?;
                while let Some(sub_entry) = sub_entries.next_entry().await? {
                    let file_name = sub_entry.file_name().to_string_lossy().to_string();
                    self.cache.insert(
                        format!(
                            "{}/{}",
                            file_or_dir_name,
                            file_name.replace(".json", "")
                        ),
                        HashMap::new(),
                    );
                }
            }

            self.cache.insert(
                entry
                    .file_name()
                    .to_string_lossy()
                    .replace(".json", "")
                    .to_string(),
                HashMap::new(),
            );
        }

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

    async fn get(&mut self, key: &RegistryKey) -> Option<NpmPackage> {
        self.perform_preload(key);

        // We have a range
        let range: Range = key.version.parse().unwrap();
        let mut selected_version: Option<NpmPackage> = None;
        for (_, v) in self.cache.get(&key.name)?.iter() {
            let v_package: Version = v.version.parse().unwrap();

            // Continue if too new or too old
            if !range.satisfies(&v_package) {
                continue;
            }

            match &mut selected_version {
                Some(sv) => {
                    let selected_version = Version::parse(&sv.version).unwrap();
                    if v_package > selected_version {
                        *sv = v.clone();
                    }
                }
                None => {
                    selected_version = Some(v.clone());
                }
            }
        }
        selected_version
    }

    async fn set(&mut self, key: &RegistryKey, value: NpmPackage) -> () {
        match self.cache.get_mut(&key.name) {
            Some(cache) => {
                cache.insert(key.version.clone(), value);
            }
            None => {
                let mut cache = HashMap::new();
                cache.insert(key.version.clone(), value);
                self.cache.insert(key.name.clone(), cache);
            }
        }
        self.persist(key).await.unwrap();
    }

    async fn has(&mut self, key: &RegistryKey) -> bool {
        self.perform_preload(key);

        self.cache.contains_key(&key.name)
            && self
                .cache
                .get(&key.name)
                .unwrap()
                .contains_key(&key.version)
    }
}
