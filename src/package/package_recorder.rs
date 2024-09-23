use crate::package::npm_package::{EnginesType, PeerDependencyMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use crate::cache::{RegistryKey, DEP_CACHE_FOLDER};
use crate::fs::get_config_dir;
use crate::package::BinType;

#[derive(Clone, Default, Debug)]
pub struct PackageMetaRecorder {
    pub name: String,
    pub version: String,
    pub resolution: Option<PackageResolution>,
    pub engines: Option<EnginesType>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub has_bin: Option<bool>,
    pub peer_dependencies_meta: Option<HashMap<String, PeerDependencyMeta>>,
    pub cpu: Option<Vec<String>>,
    pub os: Option<Vec<String>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub resolved_dependencies: Option<HashMap<String, String>>,
    pub bin: Option<BinType>,
    pub depth_traces: Option<Vec<Vec<RegistryKey>>>,
    pub resolved_binaries: Option<Vec<ResolvedBinary>>,
    // This involves transitive dependencies
    pub resolved_peer_dependencies: Option<HashMap<String, String>>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ResolvedBinary {
    pub name: String,
    pub path: String,
    pub package_name: String,
}

impl PackageMetaRecorder {
    pub fn resolve_path_to_package(&self) -> PathBuf {
        get_config_dir(DEP_CACHE_FOLDER.clone())
            .join(format!("{}-{}", self.name, self.version))
            .join("package")
    }
}


impl Display for PackageMetaRecorder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

impl From<PackageMetaRecorder> for PackageMetaHandler {
    fn from(val: PackageMetaRecorder) -> Self {
        PackageMetaHandler {
            resolution: val.resolution,
            os: val.os,
            cpu: val.cpu,
            has_bin: val.has_bin,
            engines: val.engines,
            peer_dependencies: val.peer_dependencies,
            peer_dependencies_meta: val.peer_dependencies_meta,
            dependencies: val.dependencies,
            resolved_dependencies: val.resolved_dependencies,
            bin: val.bin,
            depth_traces: val.depth_traces,
            resolved_binaries: val.resolved_binaries,
            resolved_peer_dependencies: val.resolved_peer_dependencies
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageMetaHandler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<PackageResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<EnginesType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_bin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies_meta: Option<HashMap<String, PeerDependencyMeta>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing)]
    pub resolved_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing)]
    pub resolved_peer_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<BinType>,
    #[serde(skip_serializing)]
    pub resolved_binaries: Option<Vec<ResolvedBinary>>,
    #[serde(skip_serializing)]
    pub depth_traces: Option<Vec<Vec<RegistryKey>>>
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PackageResolution {
    pub integrity: String,
}

#[derive(Clone, Default, Debug)]
pub struct PackageRecorder {
    pub main_packages: HashMap<RegistryKey,PackageMetaRecorder>,
    pub sub_dependencies: HashMap<RegistryKey,PackageMetaRecorder>,
}
