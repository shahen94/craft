use async_trait::async_trait;

use crate::{
    errors::NetworkError,
    package::{NpmPackage, Package},
};

#[async_trait]
pub trait Registry {
    async fn fetch(&self, package: &Package) -> Result<NpmPackage, NetworkError>;
}
