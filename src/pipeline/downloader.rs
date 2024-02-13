use std::{
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    cache::PackagesCache,
    contracts::{PersistentCache, Phase, Pipe, PipeArtifact, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    network::Http,
    package::NpmPackage,
};

use super::artifacts::DownloadArtifacts;

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
    pub fn new(artifacts: &dyn PipeArtifact<Vec<NpmPackage>>, tx: Sender<ProgressAction>) -> Self {
        Self {
            packages: artifacts.get_artifacts(),
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

        if self.cache.lock().await.has(&pkg.to_string()).await {
            CraftLogger::verbose(format!("Package already downloaded: {}", pkg.to_string()));
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
            Http::download_file(&pkg.dist.tarball, path).await.unwrap();

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
        let _ = self.cache.lock().await.init().await;

        let _ = self.tx.send(ProgressAction::new(Phase::Downloading));

        let _ = tokio::join! {
          async {
            CraftLogger::verbose("Downloading packages");
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

        Ok(self.artifacts.lock().await.clone())
    }
}
