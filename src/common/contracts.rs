use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;

use super::{remote_package::RemotePackage, errors::{PackageNotFoundError, GzipDownloadError, UnzipError, InstallError, UninstallError}, package::Package};

pub trait Logger {
  fn log<S: AsRef<str>>(&self, message: S);
  fn error<S: AsRef<str>>(&self, message: S);
  fn warn<S: AsRef<str>>(&self, message: S);
}

#[async_trait]
pub trait Actor {
  async fn install_package(&self, package: &Package) -> Result<(), InstallError>;
  async fn uninstall_package(&self, package: &Package) -> Result<(), UninstallError>;
  async fn update_package(&self, package: &Package);
  async fn list_packages(&self);
  async fn install_all_packages(&self);
}

pub trait PackageJson {
  fn get_dependencies(&self) -> HashMap<String, String>;
}

#[async_trait]
pub trait Registry {
  fn new(url: Option<&str>) -> Self;

  async fn get_package(&self, package: &Package) -> Result<RemotePackage, PackageNotFoundError>;
}

#[async_trait]
pub trait Modules {
  async fn download_package(&self, package: &RemotePackage) -> Result<PathBuf, GzipDownloadError>;
  async fn unzip_package(&self, package: &RemotePackage) -> Result<(), UnzipError>;

  async fn remove_package(&self, package: &str) -> Result<(), UninstallError>;

  async fn cleanup(&self);
}