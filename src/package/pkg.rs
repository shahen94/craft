use super::{
    registry::Registry,
    version::{contracts::Version, VersionImpl},
};
use crate::actors::PackageType;
use crate::cache::RegistryKey;
use std::fmt::Display;

// ─── Package ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: VersionImpl,
    pub registry: Registry,
    pub raw_version: String,
    pub package_type: PackageType,
}

impl From<Package> for RegistryKey {
    fn from(val: Package) -> Self {
        RegistryKey {
            name: val.name,
            version: val.version.to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.raw_version)
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

    pub fn new(package: PackageType) -> Self {
        let binding = package.get_name();
        let parts = binding.rsplitn(2, '@').collect::<Vec<_>>();

        match parts.len() {
            1 => Self {
                name: parts[0].to_string(),
                version: Version::new("*"),
                registry: Registry::Npm,
                raw_version: "*".to_string(),
                package_type: package,
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
                    package_type: package,
                }
            }
            _ => panic!("Invalid package name: {}", package.get_name()),
        }
    }
}

/*impl From<NpmPackage> for Package {
    fn from(pkg: NpmPackage) -> Self {
        Self {
            name: pkg.name,
            version: VersionImpl::new(&pkg.version),
            registry: Registry::Npm,
            raw_version: pkg.version,
        }
    }
}*/

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_new() {
        let package = Package::new(PackageType::Dev("lodash@4.17.21".to_string()));
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "4.17.21");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "4.17.21");

        let package = Package::new(PackageType::Optional("lodash@latest".to_string()));
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "*.*.*");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "latest");

        let package = Package::new(PackageType::Dev("lodash@*".to_string()));
        assert_eq!(package.name, "lodash");
        assert_eq!(package.version.to_string(), "*.*.*");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "*");
    }
}
