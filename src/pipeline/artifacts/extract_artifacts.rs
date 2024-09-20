use crate::fs::get_config_dir;
use crate::{cache::DEP_CACHE_FOLDER, contracts::PipeArtifact, package::NpmPackage};
use std::{collections::HashMap, path::PathBuf};
// ─────────────────────────────────────────────────────────────────────────────

pub type ExtractArtifactsMap = HashMap<String, ExtractArtifactItem>;

// ─── ExtractArtifacts ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ExtractArtifacts {
    #[allow(dead_code)]
    tmp_cache_folder: PathBuf,
    tmp_cache: ExtractArtifactsMap,
}

// ───────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ExtractArtifactItem {
    #[allow(dead_code)]
    pub package: NpmPackage,
    #[allow(dead_code)]
    pub unzip_at: PathBuf,
}

// ───────────────────────────────────────────────────────────────────────────────

impl ExtractArtifactItem {
    pub fn new(package: NpmPackage, unzip_at: PathBuf) -> Self {
        Self { package, unzip_at }
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl ExtractArtifacts {
    pub fn new() -> Self {
        let tmp_cache_folder = get_config_dir(DEP_CACHE_FOLDER.clone());
        let tmp_cache = HashMap::new();

        Self {
            tmp_cache_folder,
            tmp_cache,
        }
    }

    #[allow(dead_code)]
    pub fn to_artifact(package: NpmPackage, extracted_at: PathBuf) -> ExtractArtifactItem {
        ExtractArtifactItem::new(package, extracted_at)
    }

    pub fn add(&mut self, package: NpmPackage, unzip_at: PathBuf) {
        let item = ExtractArtifactItem::new(package.clone(), unzip_at);

        self.tmp_cache.insert(package.to_string(), item);
    }

    pub fn get(&self, package_name: &str) -> Option<&ExtractArtifactItem> {
        self.tmp_cache.get(package_name)
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl PipeArtifact<ExtractArtifactsMap> for ExtractArtifacts {
    fn get_artifacts(&self) -> ExtractArtifactsMap {
        self.tmp_cache.clone()
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

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

        let mut extract_artifacts = ExtractArtifacts::new();
        extract_artifacts.add(package.clone(), PathBuf::from("/tmp/package"));

        assert_eq!(
            extract_artifacts
                .get("package@1.0.0")
                .unwrap()
                .package
                .version,
            "1.0.0"
        );
    }

    #[test]
    fn test_get_artifacts() {
        let mut extract_artifacts = ExtractArtifacts::new();

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
        extract_artifacts.add(package.clone(), PathBuf::from("/tmp/package"));

        assert_eq!(extract_artifacts.get_artifacts().len(), 1);
    }
}
