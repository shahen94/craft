use std::sync::mpsc::Sender;

use async_recursion::async_recursion;
use async_trait::async_trait;

use crate::cache::{RegistryCache, RegistryKey};
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
    async fn resolve_pkg(
        &mut self,
        package: &Package,
        parent: Option<String>,
    ) -> Result<(), NetworkError> {
        CraftLogger::verbose(format!("Resolving package: {}", package.to_string()));

        let cached_pkg = self.cache.get(&package.clone().into()).await;

        if let Some(pkg) = cached_pkg.clone() {
            if self.artifacts.get(&pkg.to_string()).is_some() {
                CraftLogger::verbose(format!(
                    "Package found in artifacts: {}",
                    package.to_string()
                ));
            }
        }

        let final_key: RegistryKey;
        if let Some(pkg) = cached_pkg {
            CraftLogger::verbose(format!("Package found in cache: {}", package.to_string()));
            final_key = pkg.clone().into();
            self.artifacts.insert(
                pkg.to_string().clone(),
                ResolvedItem::new(pkg.clone(), parent.clone(), package.raw_version.clone()),
            );
        } else {
            let remote_package = self.npm_registry.fetch(package).await.unwrap();

            let pkg_cache_key = remote_package.to_string();
            final_key = remote_package.clone().into();
            self.artifacts.insert(
                pkg_cache_key.clone(),
                ResolvedItem::new(
                    remote_package.clone(),
                    parent.clone(),
                    package.raw_version.clone(),
                ),
            );

            self.cache
                .set(&remote_package.clone().into(), remote_package.clone())
                .await;
        }

        // This is correct because sub dependencies are always only dependencies
        for (name, version) in self.cache.get(&final_key).await.unwrap().dependencies {
            let pkg = format!("{}@{}", name, version);

            let package = Package::new(pkg.as_str());

            let parent = if let Some(ref p) = parent {
                Some(format!("{}/{}", p, final_key.name))
            } else {
                Some(final_key.name.clone())
            };
            self.resolve_pkg(&package, parent).await?;
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
