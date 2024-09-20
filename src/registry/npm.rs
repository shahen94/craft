use async_trait::async_trait;

use crate::{
    contracts::Registry,
    errors::NetworkError,
    package::{FullPackage, NpmPackage, Package},
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
                return Err(NetworkError::FailedToFetchVersion(url));
            }
        };

        Ok(remote_package)
    }
}

#[async_trait]
impl Registry for NpmRegistry {
    async fn fetch(&self, package: &Package) -> Result<NpmPackage, NetworkError> {
        log::info!("Fetching package: {}", package.to_string());

        let pkg = self.get_full_package(package).await?;
        let mut highest_satisfied_version: Option<NpmPackage> = None;

        for (version, remote_package) in pkg.versions.iter() {
            if package.satisfies(version) {
                match highest_satisfied_version {
                    Some(ref sv) => {
                        let selected_version = nodejs_semver::Version::parse(&sv.version).unwrap();
                        let current_version =
                            nodejs_semver::Version::parse(&remote_package.version).unwrap();
                        if selected_version < current_version {
                            highest_satisfied_version = Some(remote_package.clone());
                        }
                    }
                    None => {
                        highest_satisfied_version = Some(remote_package.clone());
                    }
                }
            }
        }

        if let Some(v) = highest_satisfied_version {
            return Ok(v.clone());
        }

        println!("Failed to fetch version: {}", package);

        Err(NetworkError::FailedToFetchVersion(package.to_string()))
    }
}
