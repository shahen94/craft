use async_trait::async_trait;

use crate::errors::CacheError;

#[async_trait]
pub trait PersistentCache<T> {
    async fn init(&mut self) -> Result<(), CacheError>;
    async fn clean(&self) -> Result<(), CacheError>;

    async fn has(&self, key: &str) -> bool;
    async fn get(&self, key: &str) -> Option<T>;
    async fn set(&mut self, key: &str, value: T) -> ();
}

#[async_trait]
pub trait InMemoryCache<T> {
    async fn get(&self, key: &str) -> Option<T>;
    async fn set(&mut self, key: &str, value: T) -> ();
}

#[async_trait]
pub trait CacheManager {
    async fn init(&mut self) -> ();

    async fn clean(&self) -> ();
}
