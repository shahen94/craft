use crate::{
    errors::NetworkError,
    package::{Package, RemotePackage}, contracts::PersistentCache, cache::RegistryCache,
};

const REGISTRY_URL: &str = "https://registry.npmjs.org";

#[derive(Debug)]
pub struct NpmRegistry {
    url: String,
    cache: RegistryCache,
    http: reqwest::Client,
}

impl NpmRegistry {
    pub fn new() -> Self {
        Self {
            url: REGISTRY_URL.to_string(),
            http: reqwest::Client::new(),
            cache: RegistryCache::new(),
        }
    }

    pub async fn get_package(&mut self, package: Package) -> Result<RemotePackage, NetworkError> {
        if let Some(remote_package) = self.cache.get(&format!("{}@{}", &package.name, package.version)).await {
            return Ok(remote_package);
        }

        let url = format!("{}/{}/{}", self.url, &package.name, &package.version);
        let response = self
            .http
            .get(&url)
            .header(
                "Accept",
                "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
            )
            .send()
            .await?;

        if response.status().is_success() {
            let remote_package: RemotePackage = response.json().await?;

            self.cache.set(&package.name, remote_package.clone()).await;

            return Ok(remote_package);
        }

        Err(NetworkError::FailedToFetchPackage(format!(
            "Failed to fetch package: {}@{}",
            &package.name, &package.version
        )))
    }
}
