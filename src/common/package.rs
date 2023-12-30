use super::{version::PackageVersion, errors::VersionError};
use semver::Version;


/// Single package with name and version
/// 
/// # Example
/// ```
/// let package = Package::new("lodash".to_owned(), "latest".to_owned());
/// println!("{:?}", package);
/// ```
#[derive(Debug)]
pub struct Package {
  pub name: String,
  pub version: String,
  pub parsed_version: Option<PackageVersion>,
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
    if version == "latest" {
      return Ok(Self {
        name,
        version,
        parsed_version: None,
      });
    }

    let mut version_c = version;

    // If version is 1.2 we should add a .0 to the end
    // If version is 1, we should add a .0.0 to the end
    if version_c.split(".").collect::<Vec<&str>>().len() < 3 {
      version_c = format!("{}.0.0", version_c);
    } else if version_c.split(".").collect::<Vec<&str>>().len() < 2 {
      version_c = format!("{}.0", version_c);
    }

    let parsed_version = match Version::parse(&version_c) {
      Ok(parsed_version) => Some(parsed_version),
      Err(_) => None
    };

    if parsed_version.is_none() {
      return Err(VersionError::new(
        format!("Invalid version of package {}: {}", name, version_c)
      ));
    }

    let parsed_version = parsed_version.unwrap();

    let package_version = PackageVersion::new(parsed_version.major, parsed_version.minor, parsed_version.patch);

    Ok(
      Self {
        name,
        version: version_c,
        parsed_version: Some(package_version),
      }
    )
  }
}