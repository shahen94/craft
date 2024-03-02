use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkArtifactItem {
    pub to: PathBuf,
    pub from: PathBuf,
}

// ─────────────────────────────────────────────────────────────────────────────

impl LinkArtifactItem {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }
}
