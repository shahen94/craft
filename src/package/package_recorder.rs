use crate::package::npm_package::{EnginesType, PeerDependencyMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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
    pub bin: Option<BinType>
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
            bin: val.bin
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<BinType>

}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PackageResolution {
    pub integrity: String,
}

#[derive(Clone, Default, Debug)]
pub struct PackageRecorder {
    pub main_packages: Vec<PackageMetaRecorder>,
    pub sub_dependencies: Vec<PackageMetaRecorder>,
}
