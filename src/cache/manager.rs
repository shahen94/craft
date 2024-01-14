use async_trait::async_trait;

use super::{RegistryCache, PackagesCache, graph::DependencyGraph};
use crate::{contracts::{PersistentCache, CacheManager}, package::Package};

/// CacheManager is a struct that manages all the caches in the application.
/// 
/// It is responsible for initializing all the caches and providing a single
/// interface to interact with all the caches.
/// 
/// It is also responsible for providing a single interface to interact with
/// all the caches.
/// 
/// # Example
/// 
/// ```
/// use crate::cache::CacheManager;
/// 
/// let cache_manager = CacheManager::new();
/// 
/// cache_manager.init().await.unwrap();
/// ```
#[derive(Debug)]
pub struct CacheManagerImpl {
  pub registry: RegistryCache,
  pub packages: PackagesCache,
  pub graph: DependencyGraph,
}

impl CacheManagerImpl {
  pub fn new () -> Self {
    Self {
      registry: RegistryCache::new(),
      packages: PackagesCache::new(),
      graph: DependencyGraph::new(),
    }
  }
}

#[async_trait]
impl CacheManager for CacheManagerImpl {
  async fn get_registry_cache_path(&self, key: &str) -> Option<Package> {
    self.registry.get(key).await
  }

  async fn get_packages_cache_path(&self, key: &str) -> Option<Package> {
    self.packages.get(key).await
  }

  async fn set_registry_cache_path(&self, key: &str, value: Package) -> () {
    self.registry.set(key, value).await
  }

  async fn set_packages_cache_path(&self, key: &str, value: Package) -> () {
    self.packages.set(key, value).await
  }

  async fn init(&self) -> () {
    self.registry.init().await.unwrap();
    self.packages.init().await.unwrap();
  }

  async fn clean(&self) -> () {
    self.registry.clean().await.unwrap();
    self.packages.clean().await.unwrap();
  }
}