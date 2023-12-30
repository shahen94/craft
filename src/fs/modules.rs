use std::{fs, path::PathBuf};

use async_trait::async_trait;

use crate::{
    cache::PackagesCache,
    common::{
        contracts::{Modules, PackageCaching},
        errors::{GzipDownloadError, UninstallError, UnzipError},
        remote_package::RemotePackage, Gzip,
    },
};

const TEMPORARY_FOLDER: &str = ".craft";

/// NodeModules is a struct that implements the Modules trait
#[derive(Debug)]
pub struct NodeModules {
    pub path: PathBuf,
    cache: PackagesCache,
}

impl NodeModules {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        let cache = PackagesCache::new(None);

        Self { path, cache }
    }

    pub fn init_folder(&self) {
        self.cache.init_folder();

        let craft_path = self.path.join(TEMPORARY_FOLDER);
        if !craft_path.exists() {
            std::fs::create_dir_all(craft_path).unwrap();
        }
    }
}

#[async_trait]
impl Modules for NodeModules {
    async fn download_package(
        &self,
        package: &RemotePackage,
    ) -> Result<PathBuf, GzipDownloadError> {
        let cache_path = self.cache.get(&package).await;

        if cache_path.is_some() {
            return Ok(cache_path.unwrap());
        }

        let dest = self.cache.cache(package).await?;

        return Ok(PathBuf::from(dest));
    }

    async fn unzip_package(&self, package: &RemotePackage) -> Result<(), UnzipError> {
        let archive_path = self.cache.get(&package).await;
        let unzip_folder = self.cache.get_temporary_cache_folder();
        let package_path = self.path.join(&package.name);

        if package_path.exists() {
            return Ok(());
        }

        if archive_path.is_none() {
            let error_msg = format!("Package {}@{} not found", package.name, package.version);
            return Err(UnzipError::new(error_msg));
        }

        let archive_path = archive_path.unwrap();

        Gzip::extract(&archive_path, &unzip_folder).await?;

        let unzip_folder = unzip_folder.join("package");

        match fs::rename(&unzip_folder, package_path) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error renaming folder: {:?}", error);
                return Err(UnzipError::new(error_msg));
            }
        };

        Ok(())

    }

    async fn remove_package(&self, package: &str) -> Result<(), UninstallError> {
        let path = self.path.join("node_modules").join(package);

        if !path.exists() {
            return Ok(());
        }

        match fs::remove_dir_all(path.clone()) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error removing folder: {:?}", error);
                return Err(UninstallError::new(error_msg));
            }
        };

        return Ok(());
    }

    async fn cleanup(&self) {
        let path = self.path.join(TEMPORARY_FOLDER);

        if !path.exists() {
            return;
        }

        match fs::remove_dir_all(path.clone()) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
