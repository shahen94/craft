use async_trait::async_trait;

use crate::common::{
    contracts::Registry, errors::PackageNotFoundError, package::Package,
    remote_package::RemotePackage,
};

use super::registry_cache::RegistryCache;

const REGISTRY_URL: &str = "https://registry.npmjs.org";

/// NpmRegistry is a struct that implements the Registry trait
#[derive(Debug)]
pub struct NpmRegistry {
    url: String,
    cache: RegistryCache,
}

impl NpmRegistry {
    fn get_package_url(&self, package: &Package) -> String {
        format!("{}/{}/{}", self.url, package.name, package.version)
    }
}

#[async_trait]
impl Registry for NpmRegistry {
    fn new(url: Option<&str>) -> Self {
        let url = match url {
            Some(url) => url,
            None => REGISTRY_URL,
        };

        Self {
            url: url.to_string(),
            cache: RegistryCache::new(),
        }
    }

    async fn get_package(&mut self, package: &Package) -> Result<RemotePackage, PackageNotFoundError> {
        if self.cache.has_package(&package.name) {
            let package = self.cache.get_package(&package.name).unwrap();
            return Ok(package.clone());
        }

        let url = self.get_package_url(&package);

        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let package: RemotePackage = response.json().await?;

            self.cache.add_package(&package);
            return Ok(package);
        }

        Err(PackageNotFoundError::new(format!(
            "Package {}@{} not found",
            package.name, package.version
        )))
    }
}
