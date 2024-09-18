use async_trait::async_trait;
use std::{collections::HashMap, fs::File, path::PathBuf};
use homedir::my_home;
use nodejs_semver::{Range, Version};
use crate::{contracts::PersistentCache, errors::CacheError, package::NpmPackage};

use super::constants::{REGISTRY_CACHE_FOLDER};


//
#[derive(Eq, Hash, Debug, Clone)]
pub struct RegistryKey{
    pub name: String,
    pub version: String,
}

impl From<RegistryKey> for PathBuf {
    fn from(val: RegistryKey) -> Self {
        PathBuf::from(format!("{}@{}", val.name, val.version))
    }
}

impl PartialEq for RegistryKey {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
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

impl RegistryKey {
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}

pub fn convert_to_registry_key(key: &str) -> RegistryKey {
    let key_version_arr = key.split("@").collect::<Vec<&str>>();
    RegistryKey {
        name: key_version_arr[0].to_string(),
        version: key_version_arr[1].to_string(),
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
        let cache_file = File::create(self.directory.join(format!("{}.json", key.name))).unwrap();

        serde_json::to_writer(cache_file, &self.cache.get(&key.name)).unwrap();

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Default for RegistryCache {
    fn default() -> Self {
        let directory = {
            
            my_home().unwrap().unwrap().join(REGISTRY_CACHE_FOLDER.clone())
        };

        Self {
            directory,
            cache: HashMap::new()
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl RegistryCache {
    fn load_file(&self, key: &RegistryKey) -> HashMap<String, NpmPackage> {
        // Not yet loaded into cache
        let cache_file = File::open(self.directory.join(format!("{}.json",
                                                                key.name)))
            .unwrap();

        // Loads complete configuration
        let cache: HashMap<String, NpmPackage> = serde_json::from_reader(cache_file).unwrap();
        cache
    }

    fn perform_preload(&mut self, key: &RegistryKey) {
        if let Some(cache_key) = self.cache.get(&key.name) {
            // We already have information about the package
            if cache_key.is_empty() {
                let loaded_key = self.load_file(key);
                self.cache.insert(key.name.clone(), loaded_key);
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
            self.cache.insert(entry.file_name().to_string_lossy().replace(".json", "").to_string
            (), HashMap::new());
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

        // Optimization O(1) resolve static versions
        if key.version.as_bytes()[0].is_ascii_digit() {
            // We have a specific version
            let named_package = self.cache.get(&key.name);

            named_package?.get(&key.version).cloned()
        } else {
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
            && self.cache.get(&key.name).unwrap().contains_key(&key.version)
    }
}
