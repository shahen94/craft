use semver::{Version, VersionReq};

use crate::logger::CraftLogger;

use super::errors::VersionError;

/// Single package with name and version
///
/// # Example
/// ```
/// let package = Package::new("lodash".to_owned(), "latest".to_owned());
/// println!("{:?}", package);
/// ```
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
}

impl Package {
    /// Returns the name and version of the package
    ///
    /// # Arguments
    /// * `package` - A string slice that holds the package name and version
    ///
    /// # Example
    /// ```
    /// let version = Package::get_version_from_package("lodash@latest");
    /// assert_eq!(version, "latest");
    /// ```
    pub fn parse_package(package: String) -> (String, String) {
        let parts = package.split("@").collect::<Vec<&str>>();
        let name = parts[0].to_owned();

        if parts.len() < 2 {
            return (name, "latest".to_owned());
        }

        let version = parts[1].to_owned();

        (name, version)
    }

    /// Returns a new instance of Package
    ///
    /// # Arguments
    /// * `name` - A string slice that holds the package name
    /// * `version` - A string slice that holds the package version
    ///
    /// # Example
    /// ```
    /// let package = Package::new("lodash".to_owned(), "latest".to_owned());
    /// println!("{:?}", package);
    /// ```
    pub fn new(name: String, version: String) -> Result<Self, VersionError> {
        if version == "latest" || version == "*" {
            return Ok(Self { name, version });
        }

        let data = VersionReq::parse(&version)
          .map_err(|err| {
            println!("{:?}", version);
            return VersionError::new(err.to_string());
          })?;


        let comparator = data.comparators.first()
          .ok_or_else(|| {
            println!("{:?}", version);
            return VersionError::new("Failed to get comparator".to_string());
          })?;

        let version = match (comparator.major, comparator.minor, comparator.patch) {
          (major, Some(minor), Some(patch)) => format!("{}.{}.{}", major, minor, patch),
          (major, Some(minor), None) => format!("{}.{}", major, minor),
          (major, None, None) => format!("{}", major),
          _ => return Err(VersionError::new("Failed to get version".to_string())),
        };

        Ok(Self {
            name,
            version,
        })
    }
}
