use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RemotePackage {
  pub name: String,

  pub version: String,

  #[serde(default)]
  pub dependencies: HashMap<String, String>,

  #[serde(default)]
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: HashMap<String, String>,

  pub dist: Distribution,
}

#[derive(Debug, Deserialize)]
pub struct Distribution {
  pub integrity: String,
  pub shasum: String,
  pub tarball: String,

  #[serde(rename = "fileCount")]
  pub file_count: u64,

  #[serde(rename = "unpackedSize")]
  pub unpacked_size: u64,
}