use super::{RegistryCache, PackagesCache, graph::DependencyGraph};
use crate::contracts::PersistentCache;

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
pub struct CacheManager {
  pub registry: RegistryCache,
  pub packages: PackagesCache,
  pub graph: DependencyGraph,
}

impl CacheManager {
  pub fn new () -> Self {
    Self {
      registry: RegistryCache::new(),
      packages: PackagesCache::new(),
      graph: DependencyGraph::new(),
    }
  }

  pub async fn init(&self) -> () {
    self.registry.init().await.unwrap();
    self.packages.init().await.unwrap();
  }

  pub async fn clean(&self) -> () {
    self.registry.clean().await.unwrap();
    self.packages.clean().await.unwrap();
  }
}