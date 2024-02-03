use std::collections::HashMap;

use crate::{contracts::PipeResolveArtifact, package::NpmPackage};

// ─── Artifacts ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Artifacts {
  packages: HashMap<String, NpmPackage>,
}

impl PipeResolveArtifact for Artifacts {
  fn get_artifacts(&self) -> Vec<NpmPackage> {
    self.packages.values().cloned().collect()
  }
}

// ─── Implementations ─────────────────────────────────────────────────────────

impl Artifacts {
  pub fn new() -> Self {
    Self {
      packages: HashMap::new(),
    }
  }
  
  pub fn get(&self, key: &str) -> Option<&NpmPackage> {
    self.packages.get(key)
  }

  pub fn insert(&mut self, key: String, value: NpmPackage) {
    self.packages.insert(key, value);
  }
}