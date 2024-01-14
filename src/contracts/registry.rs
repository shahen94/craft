use async_trait::async_trait;

#[async_trait]
pub trait Registry {
  async fn fetch(&self, name: &str, version: &str) -> ();
}