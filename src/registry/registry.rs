use crate::{
    cache::RegistryCache,
    contracts::PersistentCache,
    errors::NetworkError,
    package::{Package, RemotePackage, FullPackage},
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

    async fn get_exact_package(&mut self, package: Package) -> Result<RemotePackage, NetworkError> {
        let url = format!("{}/{}/{}", self.url, &package.name, &package.raw_version);
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
            return Ok(response.json().await?);
        }

        Err(NetworkError::FailedToFetchPackage(format!(
            "Failed to fetch package: {}@{}",
            &package.name, &package.raw_version
        )))
    }

    async fn get_wildcarded_package(
        &mut self,
        package: Package,
    ) -> Result<RemotePackage, NetworkError> {
        let url = format!("{}/{}", self.url, &package.name);
        let response = self
            .http
            .get(&url)
            .header(
                "Accept",
                "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
            )
            .send()
            .await?;

        if !response.status().is_success() {
          return Err(NetworkError::FailedToFetchPackage(format!(
              "Failed to fetch package: {}@{}",
              &package.name, &package.raw_version
          )));
        }

        let full_data: FullPackage = response.json().await?;

        for (version, remote_package) in full_data.versions {
            if package.satisfies(&version) {
                return Ok(remote_package);
            }
        }

        Err(NetworkError::FailedToFetchVersion(format!(
            "Failed to fetch package: {}@{}",
            &package.name, &package.raw_version
        )))
    }

    pub async fn get_package(&mut self, package: Package) -> Result<RemotePackage, NetworkError> {
      let cache_key = format!("{}@{}", &package.name, package.raw_version);

        if let Some(remote_package) = self
            .cache
            .get(&cache_key)
            .await
        {
            return Ok(remote_package);
        }

        if package.version.is_exact() {
            let remote_package = self.get_exact_package(package).await?;

            self.cache.set(&cache_key, remote_package.clone()).await;

            return Ok(remote_package);
        }

        let remote_package = self.get_wildcarded_package(package).await?;

        self.cache.set(&cache_key, remote_package.clone()).await;

        Ok(remote_package)
    }
}
