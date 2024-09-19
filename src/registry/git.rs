use async_trait::async_trait;

use crate::{
    contracts::Registry,
    errors::NetworkError,
    package::{contracts::Satisfies, FullPackage, NpmPackage, Package},
};

#[allow(unused_variables)]
#[derive(Debug)]
pub struct GitRegistry {
    #[allow(dead_code)]
    http: reqwest::Client,
}

impl GitRegistry {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }
}

impl GitRegistry {
    async fn get_archive(&self, _package: &Package) -> Result<FullPackage, NetworkError> {
        todo!()
    }
}

#[async_trait]
impl Registry for GitRegistry {
    async fn fetch(&self, package: &Package) -> Result<NpmPackage, NetworkError> {
        let pkg = self.get_archive(package).await?;

        for (version, remote_package) in pkg.versions.iter() {
                return Ok(remote_package.clone());
        }

        println!("Failed to fetch version: {}", package);

        Err(NetworkError::FailedToFetchVersion(
            package.raw_version.clone(),
        ))
    }
}
