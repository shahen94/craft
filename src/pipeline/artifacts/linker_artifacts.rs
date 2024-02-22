use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct LinkerArtifacts {
    pub artifacts: Vec<LinkArtifactItem>,
}

#[derive(Debug)]
pub struct LinkArtifactItem {
    pub path: PathBuf,
}

// ─────────────────────────────────────────────────────────────────────────────

impl LinkArtifactItem {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl LinkerArtifacts {
    pub fn add(&mut self, path: PathBuf) {
        self.artifacts.push(LinkArtifactItem::new(path));
    }
}
