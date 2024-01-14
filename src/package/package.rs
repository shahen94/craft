use semver::VersionReq;

use crate::errors::VersionError;

/// Single package with name and version
///
/// # Example
/// ```
/// let package = Package::new("lodash".to_owned(), "latest".to_owned());
/// println!("{:?}", package);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
}

impl Package {
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
    pub fn new(package: String) -> Result<Self, VersionError> {
        let (name, version) = Package::parse_package(package);

        if version == "latest" || version == "*" {
            return Ok(Self { name, version });
        }

        let data = VersionReq::parse(&version.clone()).map_err(|err| {
            println!("{:?}", version);
            return VersionError::Parse(version.clone(), name.clone(), err.to_string());
        })?;

        let comparator = data.comparators.first().ok_or_else(|| {
            return VersionError::Parse(
                version.clone(),
                name.clone(),
                "Failed to get comparator".to_string(),
            );
        })?;

        let parsed_version = match (comparator.major, comparator.minor, comparator.patch) {
            (major, Some(minor), Some(patch)) => format!("{}.{}.{}", major, minor, patch),
            (major, Some(minor), None) => format!("{}.{}", major, minor),
            (major, None, None) => format!("{}", major),
            _ => {
                return Err(VersionError::Parse(
                    version,
                    name,
                    "Failed to get version".to_string(),
                ))
            }
        };

        Ok(Self {
            name,
            version: parsed_version,
        })
    }

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
}
