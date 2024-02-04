use std::collections::HashMap;

use crate::{contracts::PipeArtifact, package::NpmPackage};

// --------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ResolveArtifacts {
  packages: HashMap<String, NpmPackage>,
}

// --------------------------------------------------------------------------------

impl ResolveArtifacts {
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

// --------------------------------------------------------------------------------

impl PipeArtifact<Vec<NpmPackage>> for ResolveArtifacts {
  fn get_artifacts(&self) -> Vec<NpmPackage> {
    self.packages.values().cloned().collect()
  }
}