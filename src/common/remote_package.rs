use std::collections::HashMap;

use serde::Deserialize;

/// This struct represents a package from the registry.
/// 
/// It is used to deserialize the JSON response from the registry.
/// 
/// # Example
/// ```
/// let package: RemotePackage = response.json().await?;
/// ```
#[derive(Debug, Deserialize, Clone)]
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

/// This struct represents the distribution of a package.
/// 
/// It is used to deserialize the JSON response from the registry.
/// 
/// # Example
/// ```
/// let package: RemotePackage = response.json().await?;
/// package.dist.integrity;
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Distribution {
  pub integrity: String,
  pub shasum: String,
  pub tarball: String,

  #[serde(rename = "fileCount")]
  pub file_count: Option<u64>,

  #[serde(rename = "unpackedSize")]
  pub unpacked_size: Option<u64>,
}