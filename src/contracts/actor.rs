use async_trait::async_trait;

#[async_trait]
pub trait Actor<T> {
    async fn start(&mut self) -> T;
}
