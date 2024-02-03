use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::{
    cache::PackagesCache,
    contracts::{PersistentCache, Pipe, PipeResolveArtifact},
    errors::ExecutionError,
    logger::CraftLogger,
    network::Network,
    package::NpmPackage,
};

// ─── DownloaderPipe ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DownloaderPipe<C: PersistentCache<PathBuf>> {
    packages: Vec<NpmPackage>,
    cache: Arc<C>,
}

// ─── Implementation ───────────────────────────────────────────────────────────

impl DownloaderPipe<PackagesCache> {
    pub fn new(artifacts: &dyn PipeResolveArtifact) -> Self {
        Self {
            packages: artifacts.get_artifacts(),
            cache: Arc::new(PackagesCache::new()),
        }
    }

    pub async fn download_pkg(&self, package: &NpmPackage) -> Result<(), ExecutionError> {
        let pkg = package.clone();

        if self.cache.has(&pkg.to_string()).await {
            return Ok(());
        }

        let cache = self.cache.clone();

        tokio::spawn(async move {
            let path = &cache.get_cache_directory().join(pkg.to_string());

            Network::download_file(&pkg.dist.tarball, path)
                .await
                .unwrap();
        })
        .await
        .unwrap();

        Ok(())
    }
}

#[async_trait]
impl Pipe<()> for DownloaderPipe<PackagesCache> {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let result = tokio::join! {
          async {
            CraftLogger::verbose("Downloading packages".to_string());
            for pkg in self.packages.iter() {

              CraftLogger::verbose(format!("Downloading package: {}", pkg.to_string()));
              match self.download_pkg(pkg).await {
                Ok(_) => {},
                Err(err) => {
                  return Err(err);
                }
              }
            }

            Ok(())
          }
        };

        result.0
    }
}
