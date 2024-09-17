use std::collections::HashMap;
use serde::{Deserialize, Serialize};

type ProjectId = String;
type ResolvedDependencies = HashMap<String, String>;
type CatalogName = String;
type DependencyName = String;
type DepPath = String;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockfileResolution {
    DirectoryResolution(DirectoryResolution),
    GitRepositoryResolution(GitRepositoryResolution),
    TarballResolution(TarballResolution),
    IntegrityResolution(IntegrityResolution)
}

/**
 * tarball hosted remotely
 */
#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TarballResolution {
    pub r#type: Option<String>,
    pub tarball: Option<String>,
    pub integrity: Option<String>,
    pub path: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DirectoryResolution {
    directory: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GitRepositoryResolution {
    repo: String,
    commit: String,
    path: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct IntegrityResolution {
    pub integrity: String
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ResolvedCatalogEntry {
    pub specifier: String,
    pub version: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PackageSnapshot {
    pub id: Option<String>,
    pub optional: Option<bool>,
    pub patched: Option<bool>,
    pub has_bin: Option<bool>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub resolution: LockfileResolution
}

pub type PackageSnapshots = HashMap<DepPath, PackageSnapshot>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PatchFile {
    pub path: String,
    pub hash: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PatchInfo {
    strict: bool,
    file: PatchFile
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LockfileSettings {
    pub auto_install_peers: Option<bool>,
    pub exclude_links_from_lockfile: Option<bool>,
    pub peers_suffix_max_length: Option<i32>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LockfileStructure {
  pub importers: Option<HashMap<ProjectId, ResolvedDependencies>>,
    pub lockfile_version: String,
    pub time: Option<HashMap<String,String>>,
    pub catalogs: Option<HashMap<CatalogName, HashMap<DependencyName, ResolvedCatalogEntry>>>,
    pub packages: Option<PackageSnapshots>,
    pub never_built_dependencies: Option<Vec<String>>,
    pub only_built_dependencies: Option<Vec<String>>,
    pub overrides: Option<HashMap<String, String>>,
    pub package_extensions_checksum: Option<String>,
    pub ignored_optional_dependencies: Option<Vec<String>>,
    pub patched_dependencies: Option<HashMap<String, PatchFile>>,
    pub pnpmfile_checksum: Option<String>,
    pub settings: Option<LockfileSettings>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct ProjectSnapshot {
    specifiers: ResolvedDependencies,
    dependencies: Option<ResolvedDependencies>,
    optional_dependencies: Option<ResolvedDependencies>,
    dev_dependencies: Option<ResolvedDependencies>,
    dependencies_meta: Option<DependenciesMeta>
}


pub type DependenciesMeta  = HashMap<DependencyName, DependencyMeta>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DependencyMeta  {
    injected: Option<bool>,
    node: Option<String>,
    patch: Option<String>
}