use crate::cache::RegistryKey;
use crate::package::package_recorder::{PackageMetaRecorder, PackageResolution};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// This struct represents a package from the registry.
///
/// It is used to deserialize the JSON response from the registry.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NpmPackage {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies_meta: Option<HashMap<String, PeerDependencyMeta>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugs: Option<Bugs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<LicenseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<BinType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<String>>,
    pub dist: Distribution,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PeerDependencyMeta {
    optional: Option<bool>,
}

impl From<NpmPackage> for PackageMetaRecorder {
    fn from(val: NpmPackage) -> Self {
        let mut meta_recoder = PackageMetaRecorder {
            name: val.name,
            version: val.version,
            peer_dependencies: val.peer_dependencies,
            peer_dependencies_meta: val.peer_dependencies_meta,
            engines: val.engines,
            ..Default::default()
        };

        if let Some(integrity) = val.dist.integrity {
            meta_recoder.resolution = Some(PackageResolution { integrity })
        }
        if val.bin.is_some() {
            meta_recoder.has_bin = Some(true)
        }

        if let Some(cpu) = val.cpu {
            meta_recoder.cpu = Some(cpu)
        }

        if let Some(os) = val.os {
            meta_recoder.os = Some(os)
        }

        meta_recoder
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repository {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum BinType {
    BinMappings(HashMap<String, String>),
    Bin(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum LicenseType {
    LicenseArray(Vec<License>),
    License(License),
    String(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    r#type: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bugs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl From<NpmPackage> for RegistryKey {
    fn from(val: NpmPackage) -> Self {
        RegistryKey {
            name: val.name,
            version: val.version,
        }
    }
}

impl PartialEq for NpmPackage {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

/// This struct represents the distribution of a package.
///
/// It is used to deserialize the JSON response from the registry.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Distribution {
    pub integrity: Option<String>,
    pub shasum: String,
    pub tarball: String,

    #[serde(rename = "fileCount")]
    pub file_count: Option<u64>,

    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<u64>,
}

impl Display for NpmPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

impl NpmPackage {
    pub fn contains_org(&self) -> bool {
        self.name.contains('/')
    }
}
