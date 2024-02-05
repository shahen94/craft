use std::{collections::HashMap, path::PathBuf};

use crate::{contracts::PipeArtifact, package::NpmPackage};

// --------------------------------------------------------------------------------

type DownloadedArtifacts = HashMap<String, StoredArtifact>;

#[derive(Debug, Clone)]
pub struct DownloadArtifacts {
    packages: DownloadedArtifacts,
}

#[derive(Debug, Clone)]
pub struct StoredArtifact {
    pub package: NpmPackage,
    pub zip_path: PathBuf,
}

// --------------------------------------------------------------------------------

impl StoredArtifact {
    pub fn new(package: NpmPackage, zip_path: PathBuf) -> Self {
        Self { package, zip_path }
    }
}

// --------------------------------------------------------------------------------

impl DownloadArtifacts {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_artifact() {
        let package = serde_json::from_str::<NpmPackage>(
            r#"
            {
                "name": "package",
                "version": "1.0.0",
                "dist": {
                    "shasum": "shasum",
                    "tarball": "https://registry.npmjs.org/package/-/package-1.0.0.tgz"
                }
            }
            "#,
        )
        .unwrap();
        let zip_path = PathBuf::from("path");
        let stored_artifact = DownloadArtifacts::to_artifact(package, zip_path);

        assert_eq!(stored_artifact.package.name, "package");
        assert_eq!(stored_artifact.zip_path, PathBuf::from("path"));
    }

    #[test]
    fn test_download_artifacts() {
        let mut download_artifacts = DownloadArtifacts::new();

        let package = serde_json::from_str::<NpmPackage>(
            r#"
            {
                "name": "package",
                "version": "1.0.0",
                "dist": {
                    "shasum": "shasum",
                    "tarball": "https://registry.npmjs.org/package/-/package-1.0.0.tgz"
                }
            }
            "#,
        )
        .unwrap();
        let stored_artifact = StoredArtifact::new(package, PathBuf::from("path"));
        download_artifacts.insert("package".to_string(), stored_artifact);

        assert_eq!(download_artifacts.get_artifacts().len(), 1);
    }
}
