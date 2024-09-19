use crate::actors::PackageType;
use crate::cache::{RegistryCache, RegistryKey};
use crate::contracts::{PersistentCache, Phase, Pipe, ProgressAction, Registry};
use crate::errors::{ExecutionError, NetworkError};
use crate::logger::CraftLogger;
use crate::package::{NpmPackage, Package, PackageRecorder};
use crate::registry::GitRegistry;
use crate::registry::NpmRegistry;
use async_recursion::async_recursion;
use async_trait::async_trait;
use futures::future;
use futures::future::join_all;
use futures::lock::Mutex;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

use super::artifacts::{ResolveArtifacts, ResolvedItem};

// ─── ResolverPipe ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ResolverPipe<C: PersistentCache<NpmPackage>> {
    packages: Vec<PackageType>,
    cache: Arc<Mutex<C>>,
    npm_registry: NpmRegistry,

    #[allow(dead_code)]
    git_registry: GitRegistry,

    artifacts: Arc<Mutex<ResolveArtifacts>>,

    tx: Sender<ProgressAction>,
}

// ─────────────────────────────────────────────────────────────────────────────

impl ResolverPipe<RegistryCache> {
    pub fn new(packages: Vec<PackageType>, tx: Sender<ProgressAction>) -> Self {
        let un_arced_cache = RegistryCache::default();
        let un_arced_articated = ResolveArtifacts::new();
        Self {
            packages,
            cache: Arc::new(Mutex::new(un_arced_cache)),
            npm_registry: NpmRegistry::new(),
            git_registry: GitRegistry::new(),
            artifacts: Arc::new(Mutex::new(un_arced_articated)),
            tx,
        }
    }

    #[async_recursion]
    async fn resolve_pkg(
        package: &Package,
        parent: Option<String>,
        package_recorder: Arc<Mutex<PackageRecorder>>,
        cache_arc: Arc<Mutex<RegistryCache>>,
        artifacts: Arc<Mutex<ResolveArtifacts>>,
    ) -> Result<(), NetworkError> {
        CraftLogger::verbose(format!("Resolving package: {}", package));
        let mut cache = { cache_arc.lock().await.clone() };

        let cached_pkg = cache.get(&package.clone().into()).await;

        if let Some(pkg) = cached_pkg.clone() {
            if artifacts.lock().await.get(&pkg.to_string()).is_some() {
                CraftLogger::verbose(format!("Package found in artifacts: {}", package));
            }
        }

        let final_key: RegistryKey;
        if let Some(pkg) = cached_pkg {
            CraftLogger::verbose(format!("Package found in cache: {}", package));
            final_key = pkg.clone().into();
            artifacts.lock().await.insert(
                pkg.to_string().clone(),
                ResolvedItem::new(
                    pkg.clone(),
                    parent.clone(),
                    package.raw_version.clone(),
                    package.package_type.clone(),
                ),
            );
        } else {
            let remote_package = NpmRegistry::new().fetch(package).await.unwrap();

            let pkg_cache_key = remote_package.to_string();
            final_key = remote_package.clone().into();
            {
                artifacts.lock().await.insert(
                    pkg_cache_key.clone(),
                    ResolvedItem::new(
                        remote_package.clone(),
                        parent.clone(),
                        package.raw_version.clone(),
                        package.package_type.clone(),
                    ),
                );
            }

            {
                let mut cache = cache_arc.lock().await;
                cache
                    .set(&remote_package.clone().into(), remote_package.clone())
                    .await;
            }
        }

        let package = {
            let mut cache = cache_arc.lock().await;
            cache.get(&final_key).await.clone().unwrap()
        };

        {
            let mut package_recorder = package_recorder.lock().await;
            match parent {
                None => {
                    package_recorder.main_packages.push(package.clone().into());
                }
                Some(_) => {
                    package_recorder
                        .sub_dependencies
                        .push(package.clone().into());
                }
            }
        }

        let mut jobs = Vec::new();
        if let Some(deps) = package.dependencies {
            // This is correct because sub dependencies are always only dependencies
            for (name, version) in deps {
                let pkg = format!("{}@{}", name, version);

                let package = Package::new(PackageType::Prod(pkg));

                let parent = if let Some(ref p) = parent {
                    Some(format!("{}/{}", p, final_key.name))
                } else {
                    Some(final_key.name.clone())
                };
                let pra = package_recorder.clone();
                let cache = cache_arc.clone();
                let artifacts = artifacts.clone();
                let handle = tokio::spawn(async move {
                    Self::resolve_pkg(&package, parent, pra, cache, artifacts).await
                });
                jobs.push(handle);
            }
        }

        let results: Vec<_> = future::join_all(jobs).await;
        // Iterate over the results
        for result in results.into_iter() {
            let jh_handle = result.unwrap();
            if let Err(e) = jh_handle {
                log::error!("Error is {}", e.to_string())
            }
        }

        Ok(())
    }

    pub async fn resolve(&self) -> Result<PackageRecorder, NetworkError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Resolving));
        let package_recorder = PackageRecorder::default();
        let package_recorder_arc = Arc::new(Mutex::new(package_recorder));

        let mut jobs = vec![];

        for pkg in self.packages.clone() {
            let pra = package_recorder_arc.clone();
            let cache = self.cache.clone();
            let artifacts = self.artifacts.clone();
            let job = tokio::spawn(async move {
                {
                    let package = Package::new(pkg);
                    Self::resolve_pkg(&package, None, pra, cache, artifacts).await
                }
            });
            jobs.push(job)
        }

        join_all(jobs).await;

        let x = Ok(package_recorder_arc.clone().lock().await.clone());
        x
    }
}

#[async_trait]
impl Pipe<(ResolveArtifacts, PackageRecorder)> for ResolverPipe<RegistryCache> {
    async fn run(&mut self) -> Result<(ResolveArtifacts, PackageRecorder), ExecutionError> {
        {
            self.cache.lock().await.init().await.unwrap();
        }

        match self.resolve().await {
            Ok(e) => {
                let artifacts = { self.artifacts.lock().await.clone() };
                Ok((artifacts, e))
            }
            Err(e) => Err(ExecutionError::JobExecutionFailed(
                "Resolve".to_owned(),
                e.to_string(),
            )),
        }
    }
}
