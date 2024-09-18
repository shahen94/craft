use crate::cache::RegistryKey;
use crate::errors::CacheError;
use async_trait::async_trait;

#[async_trait]
pub trait PersistentCache<T> {
    async fn init(&mut self) -> Result<(), CacheError>;
    async fn clean(&self) -> Result<(), CacheError>;

    async fn has(&mut self, key: &RegistryKey) -> bool;
    async fn get(&mut self, key: &RegistryKey) -> Option<T>;
    async fn set(&mut self, key: &RegistryKey, value: T) -> ();
}
