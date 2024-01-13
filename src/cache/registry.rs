use std::{collections::HashMap, sync::Arc, path::PathBuf, env};

use futures::lock::Mutex;

use crate::{common::{remote_package::RemotePackage, errors::RegistryCacheError}, logger::CraftLogger};

use super::constants::REGISTRY_CACHE_FOLDER;

const filename: &str = "cache.json";

#[derive(Debug)]
pub struct RegistryCache {
  directory: PathBuf,
  registry: Arc<Mutex<HashMap<String, RemotePackage>>>
}

impl RegistryCache {
  pub fn new(dir: Option<&str>) -> Self {

    let dir = match dir {
      Some(dir) => PathBuf::from(dir),
      None => {
        let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

        home.push_str(REGISTRY_CACHE_FOLDER);
        home.push_str("/");

        PathBuf::from(home)
      }
    };

    Self {
      directory: dir,
      registry: Arc::new(Mutex::new(HashMap::new()))
    }
  }

  /// Initializes the folder structure
  /// 
  /// Cache JSON file looks like this:
  /// {
  ///  "package-name@version": {
  ///   "name": "package-name",
  ///   "version": "version",
  ///   "dist": {
  ///   "shasum": "shasum",
  ///   "tarball": "tarball-url"
  ///   }
  /// }
  /// 
  /// If cache already exists, it will be loaded into memory
  pub async fn init_cache(&self) {
    if !self.directory.join(filename).exists() {
      CraftLogger::info("Registry cache does not exist, creating it.");
      tokio::fs::create_dir_all(&self.directory).await.unwrap();
    } else {
      CraftLogger::info("Registry cache exists, loading it into memory.");
      let registry = tokio::fs::read_to_string(&self.directory.join(filename)).await.unwrap();

      let registry: HashMap<String, RemotePackage> = serde_json::from_str(&registry).unwrap();

      *self.registry.lock().await = registry;
    }
  }

  pub async fn persist(&self) -> Result<(), RegistryCacheError> {
    let registry = self.registry.lock().await;

    let registry = serde_json::to_string(&*registry).unwrap();

    tokio::fs::write(&self.directory.join(filename), registry).await?;

    Ok(())
  }

  pub async fn get_package(&self, package_name: &str) -> Option<RemotePackage> {
    let registry = self.registry.lock().await;
    registry.get(package_name).map(|p| p.clone())
  }

  pub async fn add_package(&self, package: &RemotePackage) {
    let mut registry = self.registry.lock().await;
    registry.insert(package.name.clone(), package.clone());
  }

  pub async fn has_package(&self, package_name: &str) -> bool {
    let registry = self.registry.lock().await;
    registry.contains_key(package_name)
  }

  pub async fn remove_package(&self, package_name: &str) {
    let mut registry = self.registry.lock().await;
    registry.remove(package_name);
  }

  pub async fn clear(&self) {
    // TODO: Delete registry cache file instead of clearing the registry
    let mut registry = self.registry.lock().await;
    registry.clear();
  }
}