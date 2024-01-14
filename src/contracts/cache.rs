use async_trait::async_trait;

use crate::errors::CacheError;

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
    async fn set(&self, key: &str, value: T) -> ();
}
