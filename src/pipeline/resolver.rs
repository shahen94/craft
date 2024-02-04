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

use super::artifacts::ResolveArtifacts;

// ─── ResolverPipe ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ResolverPipe<C: PersistentCache<NpmPackage>> {
  package: String,
  cache: C,
  npm_registry: NpmRegistry,

  #[allow(dead_code)]
  git_registry: GitRegistry,

  artifacts: ResolveArtifacts,

  tx: Sender<ProgressAction>,
}

// ─────────────────────────────────────────────────────────────────────────────

impl ResolverPipe<RegistryCache> {
  pub fn new(package: String, tx: Sender<ProgressAction>) -> Self {
    Self {
      package,
      cache: RegistryCache::new(),
      npm_registry: NpmRegistry::new(),
      git_registry: GitRegistry::new(),
      artifacts: ResolveArtifacts::new(),
      tx
    }
  }

  #[async_recursion]
  async fn resolve_pkg(&mut self, package: &Package) -> Result<(), NetworkError> {
    CraftLogger::verbose(format!("Resolving package: {}", package.to_string()));

    let artifact_key = package.to_string();

    let cached_pkg = self.cache.get(&artifact_key).await;

    if let Some(_) = self.artifacts.get(&artifact_key) {
      CraftLogger::verbose(format!("Package found in artifacts: {}", package.to_string()));
      return Ok(());
    }

    if let Some(pkg) = cached_pkg {
      CraftLogger::verbose(format!("Package found in cache: {}", package.to_string()));
      self.artifacts.insert(artifact_key.clone(), pkg.clone());
      return Ok(());
    }

    let remote_package = self.npm_registry.fetch(&package).await?;

    let pkg_cache_key = remote_package.to_string();

    self.artifacts.insert(pkg_cache_key.clone(), remote_package.clone());
    self.cache.set(&pkg_cache_key, remote_package.clone()).await;

    for (name, version) in &remote_package.dependencies {
      let pkg = format!("{}@{}", name, version);

      let package = Package::new(pkg.as_str());

      self.resolve_pkg(&package).await?;
    }

    match self.cache.save().await {
      Ok(_) => (),
      Err(e) => {
        println!("Failed to save registry: {}", e);
      }
    }

    Ok(())
  }

  pub async fn resolve(&mut self) -> Result<(), NetworkError> {
    let package = Package::new(&self.package);

    let _ = self.tx.send(ProgressAction::new(Phase::Resolving));

    self.resolve_pkg(&package).await?;

    Ok(())
  }
}

#[async_trait]
impl Pipe<ResolveArtifacts> for ResolverPipe<RegistryCache> {
  async fn run(&mut self) -> Result<ResolveArtifacts, ExecutionError> {
    CraftLogger::verbose(format!("Resolving package and dependencies for: {}", self.package));

    match self.resolve().await {
      Ok(_) => Ok(self.artifacts.clone()),
      Err(e) => Err(ExecutionError::JobExecutionFailed("Resolve".to_owned(), e.to_string()))
    }
  }
}