use async_trait::async_trait;

use crate::common::{
    contracts::Registry, errors::PackageNotFoundError, package::Package,
    remote_package::RemotePackage,
};

const REGISTRY_URL: &str = "https://registry.npmjs.org";

/// NpmRegistry is a struct that implements the Registry trait
#[derive(Debug)]
pub struct NpmRegistry {
    url: String,
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
        }
    }

    async fn get_package(&self, package: &Package) -> Result<RemotePackage, PackageNotFoundError> {
        let url = self.get_package_url(&package);

        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let package: RemotePackage = response.json().await?;
            return Ok(package);
        }

        Err(PackageNotFoundError::new(format!(
            "Package {}@{} not found",
            package.name, package.version
        )))
    }
}
