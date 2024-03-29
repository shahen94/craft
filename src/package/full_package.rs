use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::NpmPackage;

// ─── FullPackage ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullPackage {
    pub versions: HashMap<String, NpmPackage>,
}
