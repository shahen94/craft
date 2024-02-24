use serde::{Deserialize, Serialize};

use crate::package::NpmPackage;

#[derive(Debug, Serialize, Deserialize)]
pub struct LockShape {
  pub package: NpmPackage,
}