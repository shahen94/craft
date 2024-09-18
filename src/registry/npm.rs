use async_trait::async_trait;

use crate::{
    contracts::Registry,
    errors::NetworkError,
    package::{
        contracts::{Satisfies, Version},
        FullPackage, NpmPackage, Package,
    },
};

const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";

#[derive(Debug)]
pub struct NpmRegistry {
    http: reqwest::Client,
}

impl NpmRegistry {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }
}

impl NpmRegistry {
    async fn get_exact_package(&self, package: &Package) -> Result<NpmPackage, NetworkError> {
        let url = format!(
            "{}/{}/{}",
            NPM_REGISTRY_URL,
            package.name,
            package.raw_version.replace('=', "").replace('*', "latest")
        );

        let response = self.http.get(&url).send().await?;

        let remote_package = response.json::<NpmPackage>().await?;

        Ok(remote_package)
    }

    async fn get_full_package(&self, package: &Package) -> Result<FullPackage, NetworkError> {
        let url = format!("{}/{}", NPM_REGISTRY_URL, package.name);

        let response = self
            .http
            .get(&url)
            .header("Accept", "application/vnd.npm.install-v1+json")
            .send()
            .await?;

        let remote_package = match response.json::<FullPackage>().await {
            Ok(pkg) => pkg,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(NetworkError::FailedToFetchVersion(
                    package.raw_version.clone(),
                ));
            }
        };

        Ok(remote_package)
    }
}

#[async_trait]
impl Registry for NpmRegistry {
    async fn fetch(&self, package: &Package) -> Result<NpmPackage, NetworkError> {
        log::info!("Fetching package: {}", package.to_string());

        if package.version.is_exact() {
            let pkg = self.get_exact_package(package).await?;

            return Ok(pkg);
        }

        let pkg = self.get_full_package(package).await?;

        for (version, remote_package) in pkg.versions.iter() {
            if package.version.satisfies(version) {
                return Ok(remote_package.clone());
            }
        }

        println!("Failed to fetch version: {}", package.to_string());

        Err(NetworkError::FailedToFetchVersion(package.to_string()))
    }
}
