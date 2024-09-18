use std::{
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

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

    pub async fn download_pkg(&self, package: &NpmPackage) -> Result<(), ExecutionError> {
        let pkg = package.clone();

        if self.cache.lock().await.has(&pkg.clone().into()).await {
            CraftLogger::verbose(format!("Package already downloaded: {}", pkg));
            let cache = self.cache.lock().await;

            self.artifacts.lock().await.insert(
                pkg.to_string(),
                DownloadArtifacts::to_artifact(
                    pkg.clone(),
                    cache.get_cache_directory().join(pkg.to_string()),
                ),
            );

            return Ok(());
        }

        let cache = self.cache.clone();
        let artifacts = self.artifacts.clone();

        tokio::spawn(async move {
            let path = &cache
                .lock()
                .await
                .get_cache_directory()
                .join(pkg.to_string());

            if pkg.contains_org() {
                Self::prepare_pkg_for_download(path).await.unwrap();
            }
            let result = Http::download_file(&pkg.dist.tarball, path, &pkg.dist.shasum).await;
            if result.is_err() {
                CraftLogger::error(format!("Failed to download package: {}", pkg));
                return;
            }

            artifacts.lock().await.insert(
                pkg.to_string(),
                DownloadArtifacts::to_artifact(pkg.clone(), path.clone()),
            );
        })
        .await
        .unwrap();

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

        let _ = tokio::join! {
          async {
            CraftLogger::verbose("Downloading packages");
            for pkg in self.packages.iter() {

              CraftLogger::verbose(format!("Downloading package: {}", pkg));
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

        Ok(self.artifacts.lock().await.clone())
    }
}
