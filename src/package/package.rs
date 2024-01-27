use semver::{VersionReq, Version};

use super::info::VersionInfo;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: VersionInfo,
    pub raw_version: String,
}

impl Package {
  pub fn new(package: &str) -> Self {
      let parts = package.rsplitn(2, '@').collect::<Vec<_>>();

      match parts.len() {
          1 => Self {
              name: parts[0].to_string(),
              version: VersionInfo::new("*"),
              raw_version: "*".to_string(),
          },
          2 => {
            let escaped_version = if parts[0] == "latest" {
              "*".to_string()
            } else {
              parts[0].to_string()
            };

            return Self {
              name: if package.starts_with("@") {
                  format!("{}", parts[1])
              } else {
                  parts[1].to_string()
              },

              version:VersionInfo::new(&escaped_version),
              raw_version: parts[0].to_string(),
          };
          },
          _ => panic!("Invalid package name: {}", package),
      }
  }

  pub fn satisfies(&self, version: &str) -> bool {
    // self.version.matches(&Version::parse(version).expect("Invalid version"))
    false
  }
}