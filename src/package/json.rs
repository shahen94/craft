use std::collections::HashMap;

use serde::Deserialize;

// ─── PackageJson ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub dependencies: HashMap<String, String>,
}

// ─────────────────────────────────────────────────────────────────────────────

impl From<String> for PackageJson {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}
