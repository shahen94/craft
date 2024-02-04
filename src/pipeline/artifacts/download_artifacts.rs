use std::{collections::HashMap, path::PathBuf};

use crate::{contracts::PipeArtifact, package::NpmPackage};

// --------------------------------------------------------------------------------

type DownloadedArtifacts = HashMap<String, StoredArtifact>;

#[derive(Debug, Clone)]
pub struct DownloadArtifacts {
  packages: DownloadedArtifacts
}

#[derive(Debug, Clone)]
pub struct StoredArtifact {
  pub package: NpmPackage,
  pub zip_path: PathBuf,
}

// --------------------------------------------------------------------------------

impl StoredArtifact {
  pub fn new(package: NpmPackage, zip_path: PathBuf) -> Self {
    Self {
      package,
      zip_path
    }
  }
}

// --------------------------------------------------------------------------------

impl DownloadArtifacts {
  pub fn new() -> Self {
    Self {
      packages: HashMap::new()
    }
  }

  pub fn to_artifact(package: NpmPackage, zip_path: PathBuf) -> StoredArtifact {
    StoredArtifact::new(package, zip_path)
  }

  pub fn insert(&mut self, key: String, value: StoredArtifact) {
    self.packages.insert(key, value);
  }
}

// --------------------------------------------------------------------------------

impl PipeArtifact<Vec<StoredArtifact>> for DownloadArtifacts {
  fn get_artifacts(&self) -> Vec<StoredArtifact> {
    self.packages.values().cloned().collect()
  }
}