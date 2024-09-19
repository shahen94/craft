use std::{path::{Path, PathBuf}, sync::{mpsc::Sender, Arc}};
use futures::future;

use async_trait::async_trait;
use tokio::sync::Mutex;

use super::artifacts::{DownloadArtifacts, ResolvedItem};
use crate::contracts::Logger;
use crate::{
    cache::PackagesCache,
    contracts::{PersistentCache, Phase, Pipe, PipeArtifact, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    network::Http,
    package::NpmPackage,
};

// ─── DownloaderPipe ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DownloaderPipe<C: PersistentCache<PathBuf>> {
    packages: Vec<NpmPackage>,
    cache: Arc<Mutex<C>>,
    artifacts: Arc<Mutex<DownloadArtifacts>>,
    tx: Sender<ProgressAction>,
}

// ─── Implementation ───────────────────────────────────────────────────────────

impl DownloaderPipe<PackagesCache> {
    pub fn new(
        artifacts: &dyn PipeArtifact<Vec<ResolvedItem>>,
        tx: Sender<ProgressAction>,
    ) -> Self {
        Self {
            packages: artifacts
                .get_artifacts()
                .iter()
                .map(|item| item.package.clone())
                .collect(),
            cache: Arc::new(Mutex::new(PackagesCache::default())),
            artifacts: Arc::new(Mutex::new(DownloadArtifacts::new())),
            tx,
        }
    }

    async fn prepare_pkg_for_download(download_path: &Path) -> Result<(), std::io::Error> {
        tokio::fs::create_dir_all(download_path.parent().unwrap()).await
    }

    pub async fn download_pkg(package: &NpmPackage, mut cache: PackagesCache, artifacts: Arc<Mutex<DownloadArtifacts>>) -> Result<(), ExecutionError> {
        let pkg = package.clone();

        if cache.has(&pkg.clone().into()).await {
            CraftLogger::verbose(format!("Package already downloaded: {}", pkg));
            let cache_dir = {
                cache.get_cache_directory().join(pkg.to_string())
            };

            {
                artifacts.lock().await.insert(
                    pkg.to_string(),
                    DownloadArtifacts::to_artifact(
                        pkg.clone(),
                        cache_dir,
                    ),
                );
            }

            return Ok(());
        }

        let path = {
            &cache
                .get_cache_directory()
                .join(pkg.to_string())
        };

        if pkg.contains_org() {
            Self::prepare_pkg_for_download(path).await.unwrap();
        }
        let result = Http::download_file(&pkg.dist.tarball, path, &pkg.dist.shasum).await;
        if result.is_err() {
            CraftLogger::warn(format!("Failed to download package: {}", pkg));
            return Err(ExecutionError::JobExecutionFailed("Failed to download package".parse().unwrap(), "Failed to download package".parse().unwrap()));
        }

        {
            artifacts.lock().await.insert(
                pkg.to_string(),
                DownloadArtifacts::to_artifact(pkg.clone(), path.clone()),
            );
        }

        Ok(())
    }
}

#[async_trait]
impl Pipe<DownloadArtifacts> for DownloaderPipe<PackagesCache> {
    async fn run(&mut self) -> Result<DownloadArtifacts, ExecutionError> {
        {
            let mut cache = self.cache.lock().await;
            cache
                .init()
                .await
                .map_err(|e| ExecutionError::JobExecutionFailed(e.to_string(), e.to_string()))?;
        }

        let _ = self.tx.send(ProgressAction::new(Phase::Downloading));

        let mut jobs = vec![];

        let pkgs = self.packages.clone();
        let cache = {
            self.cache.lock().await.clone()
        };

        for pkg in pkgs {
            let cache = cache.clone();
            let artifacts = self.artifacts.clone();
            let job = tokio::spawn(async move {
                CraftLogger::verbose(format!("Downloading package: {}", pkg));
                return match Self::download_pkg(&pkg, cache, artifacts).await {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(err)
                    }
                };
            });
            jobs.push(job);
        }

        let results: Vec<_> = future::join_all(jobs).await;
        // Iterate over the results
        for result in results.into_iter() {
            let jh_handle = result.unwrap();
            if let Err(e) = jh_handle {
                log::error!("Error is {}",e.to_string())
            }
        }


        Ok(self.artifacts.lock().await.clone())
    }
}
