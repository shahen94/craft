use super::registry::Registry;
use crate::actors::PackageType;
use crate::cache::RegistryKey;
use nodejs_semver::Range;
use std::fmt::Display;
// ─── Package ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub registry: Registry,
    pub raw_version: String,
    pub package_type: PackageType,
}

impl From<Package> for RegistryKey {
    fn from(val: Package) -> Self {
        RegistryKey {
            name: val.name,
            version: val.raw_version.to_string(),
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
    pub(crate) fn satisfies(&self, version: &str) -> bool {
        let range: Range = self.raw_version.parse().unwrap();
        let version: nodejs_semver::Version = version.parse().unwrap();
        version.satisfies(&range)
    }

    pub fn new(package: PackageType) -> Self {
        let binding = package.get_parts();

        Self {
            name: binding.0,
            registry: Registry::Npm,
            raw_version: binding.1,
            package_type: package,
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
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "4.17.21");

        let package = Package::new(PackageType::Optional("lodash@latest".to_string()));
        assert_eq!(package.name, "lodash");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "latest");

        let package = Package::new(PackageType::Dev("lodash@*".to_string()));
        assert_eq!(package.name, "lodash");
        assert_eq!(package.registry, Registry::Npm);
        assert_eq!(package.raw_version, "*");
    }
}
