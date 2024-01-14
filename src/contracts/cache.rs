use async_trait::async_trait;

use crate::{errors::CacheError, package::Package};

#[async_trait]
pub trait PersistentCache<T> {
    async fn init(&self) -> Result<(), CacheError>;
    async fn clean(&self) -> Result<(), CacheError>;

    async fn get(&self, key: &str) -> Option<T>;
    async fn set(&self, key: &str, value: T) -> ();
}

#[async_trait]
pub trait InMemoryCache<T> {
    async fn get(&self, key: &str) -> Option<T>;
    async fn set(&mut self, key: &str, value: T) -> ();
}


#[async_trait]
pub trait CacheManager {
    async fn init(&self) -> ();

    async fn get_registry_cache_path(&self, key: &str) -> Option<Package>;
    async fn get_packages_cache_path(&self, key: &str) -> Option<Package>;

    async fn set_registry_cache_path(&self, key: &str, value: Package) -> ();
    async fn set_packages_cache_path(&self, key: &str, value: Package) -> ();

    async fn clean(&self) -> ();
}