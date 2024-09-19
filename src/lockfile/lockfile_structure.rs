use crate::package::PackageMetaHandler;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResolvedDependency {
    pub specifier: String,
    pub version: String,
}

type ProjectId = String;
pub type ResolvedDependencies = HashMap<String, ResolvedDependency>;
type CatalogName = String;
type DependencyName = String;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockfileResolution {
    Directory(DirectoryResolution),
    GitRepository(GitRepositoryResolution),
    Tarball(TarballResolution),
    Integrity(IntegrityResolution),
}

/**
 * tarball hosted remotely
 */
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TarballResolution {
    pub r#type: Option<String>,
    pub tarball: Option<String>,
    pub integrity: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryResolution {
    directory: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRepositoryResolution {
    repo: String,
    commit: String,
    path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityResolution {
    pub integrity: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedCatalogEntry {
    pub specifier: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSnapshot {
    pub id: Option<String>,
    pub optional: Option<bool>,
    pub patched: Option<bool>,
    pub has_bin: Option<bool>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub resolution: LockfileResolution,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchFile {
    pub path: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchInfo {
    strict: bool,
    file: PatchFile,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockfileSettings {
    pub auto_install_peers: Option<bool>,
    pub exclude_links_from_lockfile: Option<bool>,
    pub peers_suffix_max_length: Option<i32>,
}

fn default_lockfile_version() -> String {
    "9.0".to_string()
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImporterSections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<ResolvedDependencies>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockfileStructure {
    #[serde(default = "default_lockfile_version")]
    pub lockfile_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LockfileSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importers: Option<HashMap<ProjectId, ImporterSections>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<HashMap<CatalogName, HashMap<DependencyName, ResolvedCatalogEntry>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<HashMap<String, PackageMetaHandler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_built_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_built_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_extensions_checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_optional_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patched_dependencies: Option<HashMap<String, PatchFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnpmfile_checksum: Option<String>,
}

impl Default for LockfileStructure {
    fn default() -> Self {
        LockfileStructure {
            lockfile_version: "9.0".to_string(),
            importers: None,
            ignored_optional_dependencies: None,
            overrides: None,
            catalogs: None,
            only_built_dependencies: None,
            package_extensions_checksum: None,
            settings: None,
            time: None,
            patched_dependencies: None,
            pnpmfile_checksum: None,
            never_built_dependencies: None,
            packages: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectSnapshot {
    specifiers: ResolvedDependencies,
    dependencies: Option<ResolvedDependencies>,
    optional_dependencies: Option<ResolvedDependencies>,
    dev_dependencies: Option<ResolvedDependencies>,
    dependencies_meta: Option<DependenciesMeta>,
}

pub type DependenciesMeta = HashMap<DependencyName, DependencyMeta>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyMeta {
    injected: Option<bool>,
    node: Option<String>,
    patch: Option<String>,
}
