use crate::{
    cache::RegistryCache,
    common::{
        errors::PackageNotFoundError, package::Package,
        remote_package::RemotePackage,
    },
};

const REGISTRY_URL: &str = "https://registry.npmjs.org";

/// NpmRegistry is a struct that implements the Registry trait
#[derive(Debug)]
pub struct NpmRegistry {
    url: String,
}

impl NpmRegistry {
    pub fn new(url: Option<&str>) -> Self {
        let url = match url {
            Some(url) => url,
            None => REGISTRY_URL,
        };

        Self {
            url: url.to_string(),
        }
    }

    fn get_package_url(&self, package: &Package) -> String {
        format!("{}/{}/{}", self.url, package.name, package.version)
    }
}

impl NpmRegistry {
    pub async fn get_package(
        &mut self,
        package: &Package,
        cache: &RegistryCache
    ) -> Result<RemotePackage, PackageNotFoundError> {
        if cache.has_package(&package.name).await {
            let package = cache.get_package(&package.name).await.unwrap();
            return Ok(package.clone());
        }

        let url = self.get_package_url(&package);

        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let package: RemotePackage = response.json().await?;

            cache.add_package(&package).await;
            return Ok(package);
        }

        Err(PackageNotFoundError::new(format!(
            "Package {}@{} not found",
            package.name, package.version
        )))
    }
}
