use super::{constraint::VersionConstraint, contracts::Satisfies};

// ─── VersionField ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum VersionField {
    Exact(u64),
    Wildcard,
}

// ─────────────────────────────────────────────────────────────────────────────

impl ToString for VersionField {
    fn to_string(&self) -> String {
        match self {
            VersionField::Exact(value) => format!("{}", value),
            VersionField::Wildcard => "*".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Satisfies for VersionField {
    fn satisfies(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }

                let version = VersionConstraint::parse(version);

                match version.major {
                    VersionField::Exact(major) => {
                        if major != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                match version.minor {
                    VersionField::Exact(minor) => {
                        if minor != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                match version.patch {
                    VersionField::Exact(patch) => {
                        if patch != *value {
                            return false;
                        }
                    }
                    _ => {}
                }

                true
            }
            VersionField::Wildcard => true,
        }
    }
}