use std::collections::HashMap;

use serde::Deserialize;

// ─── PackageJson ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PackageJson {
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub optional_dependencies: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,
}

// ─────────────────────────────────────────────────────────────────────────────

impl From<String> for PackageJson {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}
