use std::sync::mpsc::Sender;

use async_recursion::async_recursion;
use async_trait::async_trait;

use crate::cache::RegistryCache;
use crate::contracts::{PersistentCache, Phase, Pipe, ProgressAction, Registry};
use crate::errors::{ExecutionError, NetworkError};
use crate::logger::CraftLogger;
use crate::package::{NpmPackage, Package};
use crate::registry::GitRegistry;
use crate::registry::NpmRegistry;

use super::artifacts::{ResolveArtifacts, ResolvedItem};

// ─── ResolverPipe ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ResolverPipe<C: PersistentCache<NpmPackage>> {
    packages: Vec<String>,
    cache: C,
    npm_registry: NpmRegistry,

    #[allow(dead_code)]
    git_registry: GitRegistry,

    artifacts: ResolveArtifacts,

    tx: Sender<ProgressAction>,
}

// ─────────────────────────────────────────────────────────────────────────────

impl ResolverPipe<RegistryCache> {
    pub fn new(packages: Vec<String>, tx: Sender<ProgressAction>) -> Self {
        Self {
            packages,
            cache: RegistryCache::default(),
            npm_registry: NpmRegistry::new(),
            git_registry: GitRegistry::new(),
            artifacts: ResolveArtifacts::new(),
            tx,
        }
    }

    #[async_recursion]
    async fn resolve_pkg(&mut self, package: &Package, parent: Option<String>) -> Result<(), NetworkError> {
        CraftLogger::verbose(format!("Resolving package: {}", package.to_string()));

        let artifact_key = package.to_string();

        let cached_pkg = self.cache.get(&artifact_key).await;

        if self.artifacts.get(&artifact_key).is_some() {
            CraftLogger::verbose(format!(
                "Package found in artifacts: {}",
                package.to_string()
            ));
            return Ok(());
        }

        if let Some(pkg) = cached_pkg {
            CraftLogger::verbose(format!("Package found in cache: {}", package.to_string()));
            self.artifacts.insert(artifact_key.clone(), ResolvedItem::new(pkg.clone(), parent));
            return Ok(());
        }

        let remote_package = self.npm_registry.fetch(package).await.unwrap();

        let pkg_cache_key = remote_package.to_string();

        self.artifacts
            .insert(pkg_cache_key.clone(), ResolvedItem::new(remote_package.clone(), parent.clone()));

        self.cache.set(&pkg_cache_key, remote_package.clone()).await;

        for (name, version) in &remote_package.dependencies {
            let pkg = format!("{}@{}", name, version);

            let package = Package::new(pkg.as_str());

            let parent = if let Some(ref p) = parent {
                Some(format!("{}/{}", p, remote_package.name))
            } else {
                Some(remote_package.name.clone())
            };
            self.resolve_pkg(&package, parent).await?;
        }

        match self.cache.persist().await {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to save registry: {}", e);
            }
        }

        Ok(())
    }

    pub async fn resolve(&mut self) -> Result<(), NetworkError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Resolving));

        for pkg in self.packages.clone() {
            let package = Package::new(pkg.as_str());

            self.resolve_pkg(&package, None).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl Pipe<ResolveArtifacts> for ResolverPipe<RegistryCache> {
    async fn run(&mut self) -> Result<ResolveArtifacts, ExecutionError> {
        self.cache.init().await.unwrap();

        match self.resolve().await {
            Ok(_) => Ok(self.artifacts.clone()),
            Err(e) => Err(ExecutionError::JobExecutionFailed(
                "Resolve".to_owned(),
                e.to_string(),
            )),
        }
    }
}
