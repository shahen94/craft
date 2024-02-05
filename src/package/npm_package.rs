use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// This struct represents a package from the registry.
///
/// It is used to deserialize the JSON response from the registry.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NpmPackage {
    pub name: String,

    pub version: String,

    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,

    pub dist: Distribution,
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

impl ToString for NpmPackage {
    fn to_string(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}
