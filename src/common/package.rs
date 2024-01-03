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
    if version == "latest" {
      return Ok(Self {
        name,
        version,
      });
    }

    println!("Version: {}", version);
    let escaped_version = version
      .replace("^", "")
      .replace("~", "")
      .replace(">", "")
      .replace("<", "")
      .replace("=", "")
      .replace(" ", "")
      .replace("*", "0")
      .replace(".x", ".0")
      .replace("x.", "0");

    Ok(
      Self {
        name,
        version: escaped_version,
      }
    )
  }
}