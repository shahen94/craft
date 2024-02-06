use crate::logger::CraftLogger;

use super::{
    registry::Registry,
    version::{contracts::Version, VersionImpl},
};

// ─── Package ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: VersionImpl,
    pub registry: Registry,
    pub raw_version: String,
}

// ─────────────────────────────────────────────────────────────────────────────

impl ToString for Package {
    fn to_string(&self) -> String {
        format!("{}@{}", self.name, self.raw_version)
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Package {
    fn detect_registry(version: &str) -> Registry {
        if Registry::is_git(version) {
            return Registry::Git;
        }

        Registry::Npm
    }

    pub fn new(package: &str) -> Self {
        CraftLogger::verbose(format!("Parsing package: {}", package));

        let parts = package.rsplitn(2, '@').collect::<Vec<_>>();

        match parts.len() {
            1 => Self {
                name: parts[0].to_string(),
                version: Version::new("*"),
                registry: Registry::Npm,
                raw_version: "*".to_string(),
            },
            2 => {
                let escaped_version = if parts[0] == "latest" {
                    "*".to_string()
                } else {
                    parts[0].to_string()
                };

                let registry = Self::detect_registry(&escaped_version);
                let version = match registry {
                    Registry::Npm => VersionImpl::new(&escaped_version),
                    Registry::Git => VersionImpl::new("*"),
                };

                Self {
                    name: parts[1].to_string(),
                    registry,
                    version,
                    raw_version: parts[0].to_string(),
                }
            }
            _ => panic!("Invalid package name: {}", package),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_new() {
        let package = Package::new("lodash@4.17.21");
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "4.17.21");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "4.17.21");

        let package = Package::new("lodash@latest");
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "*.*.*");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "latest");

        let package = Package::new("lodash@*");
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "*.*.*");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "*");
    }
}
