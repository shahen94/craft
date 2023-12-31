use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;

use super::{
    errors::{GzipDownloadError, UninstallError, UnzipError},
    remote_package::RemotePackage,
};

/// PackageJson trait for reading package.json files
///
/// # Example
///
/// ```
/// use craft::common::contracts::PackageJson;
/// use std::collections::HashMap;
///
/// struct PackageJsonFile {
///  dependencies: HashMap<String, String>
/// }
///
/// impl PackageJson for PackageJsonFile {
///   fn get_dependencies(&self) -> HashMap<String, String> {
///     self.dependencies.clone()
///   }
/// }
/// ```
pub trait PackageJson {
    fn get_dependencies(&self) -> HashMap<String, String>;
}

/// Modules trait for downloading, unzipping and removing packages
///
/// # Example
///
/// ```
/// use craft::common::contracts::Modules;
/// use craft::common::remote_package::RemotePackage;
/// use craft::common::errors::{GzipDownloadError, UnzipError, UninstallError};
/// use async_trait::async_trait;
/// use std::path::PathBuf;
///
/// struct NodeModules;
///
/// impl Modules for NodeModules {
///   async fn download_package(&self, package: &RemotePackage) -> Result<PathBuf, GzipDownloadError> {
///     println!("Downloading package {}", package.name);
///     Ok(PathBuf::from("./node_modules"))
///   }
///
///   async fn unzip_package(&self, package: &RemotePackage) -> Result<(), UnzipError> {
///     println!("Unzipping package {}", package.name);
///     Ok(())
///   }
///
///   async fn remove_package(&self, package: &str) -> Result<(), UninstallError> {
///     println!("Removing package {}", package);
///     Ok(())
///   }
///
///   async fn cleanup(&self) {
///     println!("Cleaning up");
///   }
/// }
/// ```
#[async_trait]
pub trait Modules {
    async fn download_package(&self, package: &RemotePackage)
        -> Result<PathBuf, GzipDownloadError>;
    async fn unzip_package(&self, package: &RemotePackage) -> Result<(), UnzipError>;

    async fn remove_package(&self, package: &str) -> Result<(), UninstallError>;

    async fn cleanup(&self);
}
