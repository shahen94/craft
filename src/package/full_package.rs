use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RemotePackage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullPackage {
  pub versions: HashMap<String, RemotePackage>,
}