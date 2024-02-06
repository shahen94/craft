use std::{collections::HashMap, env, path::PathBuf};

use crate::{cache::TMP_CACHE_FOLDER, contracts::PipeArtifact, package::NpmPackage};

// ─── ExtractArtifacts ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ExtractArtifacts {
    #[allow(dead_code)]
    tmp_cache_folder: PathBuf,
    tmp_cache: HashMap<String, ExtractArtifactItem>,
}

// ───────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ExtractArtifactItem {
    #[allow(dead_code)]
    package: NpmPackage,
    #[allow(dead_code)]
    unzip_at: PathBuf,
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
        let tmp_cache_folder = Self::get_tmp_folder();
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

    pub fn get_tmp_folder() -> PathBuf {
        let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        home.push_str(TMP_CACHE_FOLDER);

        PathBuf::from(home)
    }

    pub fn add(&mut self, package: NpmPackage, unzip_at: PathBuf) {
        let name = package.name.clone();
        let item = ExtractArtifactItem::new(package.clone(), unzip_at);

        self.tmp_cache.insert(name, item);
    }

    pub fn get(&self, package_name: &str) -> Option<&ExtractArtifactItem> {
        self.tmp_cache.get(package_name)
    }
}

// ───────────────────────────────────────────────────────────────────────────────

impl PipeArtifact<Vec<ExtractArtifactItem>> for ExtractArtifacts {
    fn get_artifacts(&self) -> Vec<ExtractArtifactItem> {
        self.tmp_cache.values().cloned().collect()
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
            extract_artifacts.get("package").unwrap().package.version,
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
