use super::contracts::Satisfies;

// ─── VersionField ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum VersionField {
    Exact(u64),
    Wildcard,
}

impl VersionField {
    pub fn is_gt(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }
                let version = version.parse::<u64>().unwrap();
                version > *value
            }
            VersionField::Wildcard => true,
        }
    }

    pub fn is_gte(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }
                let version = version.parse::<u64>().unwrap();
                version >= *value
            }
            VersionField::Wildcard => true,
        }
    }

    pub fn is_lt(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }
                let version = version.parse::<u64>().unwrap();
                version < *value
            }
            VersionField::Wildcard => true,
        }
    }

    pub fn is_lte(&self, version: &str) -> bool {
        match self {
            VersionField::Exact(value) => {
                if version == "*" || version == "x" || version == "latest" {
                    return true;
                }
                let version = version.parse::<u64>().unwrap();
                version <= *value
            }
            VersionField::Wildcard => true,
        }
    }
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
                let version = version.parse::<u64>().unwrap();

                version == *value
            }
            VersionField::Wildcard => true,
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_field_to_string() {
        let version_field = VersionField::Exact(1);
        assert_eq!(version_field.to_string(), "1".to_string());

        let version_field = VersionField::Wildcard;
        assert_eq!(version_field.to_string(), "*".to_string());
    }

    #[test]
    fn test_version_field_satisfies() {
        let version_field = VersionField::Exact(1);
        assert!(version_field.satisfies("1"));
        assert!(!version_field.satisfies("2"));
        assert!(version_field.satisfies("*"));
        assert!(version_field.satisfies("x"));
        assert!(version_field.satisfies("latest"));

        let version_field = VersionField::Wildcard;
        assert!(version_field.satisfies("1"));
        assert!(version_field.satisfies("2"));
        assert!(version_field.satisfies("*"));
        assert!(version_field.satisfies("x"));
        assert!(version_field.satisfies("latest"));
    }
}
