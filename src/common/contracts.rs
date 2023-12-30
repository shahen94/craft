use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;

use super::{
    errors::{GzipDownloadError, InstallError, PackageNotFoundError, UninstallError, UnzipError},
    package::Package,
    remote_package::RemotePackage,
};

/// Logger trait for logging messages
///
/// # Example
/// ```
/// use craft::common::contracts::Logger;
///
/// struct ConsoleLogger;
///
/// impl Logger for ConsoleLogger {
///  fn log<S: AsRef<str>>(&self, message: S) {
///   println!("{}", message.as_ref());
/// }
///
/// fn error<S: AsRef<str>>(&self, message: S) {
///  println!("{}", message.as_ref());
/// }
///
/// fn warn<S: AsRef<str>>(&self, message: S) {
///   println!("{}", message.as_ref());
/// }
///
/// }
pub trait Logger {
    fn log<S: AsRef<str>>(&self, message: S);
    fn error<S: AsRef<str>>(&self, message: S);
    fn warn<S: AsRef<str>>(&self, message: S);
}

/// Actor trait for installing, uninstalling, updating and listing packages
/// 
/// # Example
/// 
/// ```
/// use craft::common::contracts::Actor;
/// use craft::common::package::Package;
/// use craft::common::errors::InstallError;
/// use async_trait::async_trait;
/// 
/// struct InstallActions;
/// 
/// impl Actor for InstallActions {
///   async fn install_package(&self, package: &Package) -> Result<(), InstallError> {
///     println!("Installing package {}", package.name);
///     Ok(())
///   }
/// 
///   async fn uninstall_package(&self, package: &Package) -> Result<(), InstallError> {
///     println!("Uninstalling package {}", package.name);
///     Ok(())
///   }
/// 
///   async fn update_package(&self, package: &Package) {
///     println!("Updating package {}", package.name);
///   }
/// 
///   async fn list_packages(&self) {
///     println!("Listing packages");
///   }
/// 
///   async fn install_all_packages(&self) {
///     println!("Installing all packages");
///   }
/// }
/// ```
#[async_trait]
pub trait Actor {
    async fn install_package(&self, package: &Package) -> Result<(), InstallError>;
    async fn uninstall_package(&self, package: &Package) -> Result<(), UninstallError>;
    async fn update_package(&self, package: &Package);
    async fn list_packages(&self);
    async fn install_all_packages(&self);
}

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

#[async_trait]
pub trait Registry {
    fn new(url: Option<&str>) -> Self;

    async fn get_package(&self, package: &Package) -> Result<RemotePackage, PackageNotFoundError>;
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
