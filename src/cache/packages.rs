use std::{env, path::PathBuf};

use async_trait::async_trait;
use crate::common::{
    contracts::PackageCaching,
    errors::GzipDownloadError,
    remote_package::RemotePackage,
    Downloader,
};

#[derive(Debug)]
pub struct PackagesCache {
    directory: PathBuf,
}

impl PackagesCache {
    pub fn new(directory: Option<&str>) -> Self {
        let directory = match directory {
            Some(directory) => PathBuf::from(directory),
            None => {
                let mut home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
                home.push_str("/.craft/cache/packages");

                PathBuf::from(home)
            }
        };

        let directory = PathBuf::from(directory);

        Self { directory }
    }

    pub fn init_folder(&self) {
        if !self.directory.exists() {
            std::fs::create_dir_all(&self.directory).unwrap();
        }

        let temporary_folder = self.get_temporary_cache_folder();

        if !temporary_folder.exists() {
            std::fs::create_dir_all(temporary_folder).unwrap();
        }
    }

    pub fn cleanup_temporary_cache_folder(&self) {
        let temporary_folder = self.get_temporary_cache_folder();

        if temporary_folder.exists() {
            std::fs::remove_dir_all(temporary_folder).unwrap();
        }
    }

    pub fn get_temporary_cache_folder(&self) -> PathBuf {
        let mut temporary_folder = self.directory.clone();
        temporary_folder.push(".temporary");

        temporary_folder
    }

    pub fn get_cached_remote_package_path(&self, package: &RemotePackage) -> PathBuf {
        self.directory.join(
          format!(
            "{}-{}-{}.tgz",
            package.name, package.version, package.dist.shasum
          )
        )
    }

    pub async fn force_clean(&self) -> Result<(), GzipDownloadError> {
        let temporary_folder = self.get_temporary_cache_folder();

        if temporary_folder.exists() {
            tokio::fs::remove_dir_all(temporary_folder).await?;
        }

        if self.directory.exists() {
            tokio::fs::remove_dir_all(&self.directory).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl PackageCaching for PackagesCache {
    async fn cache(&self, package: &RemotePackage) -> Result<PathBuf, GzipDownloadError> {
        let package_path = self.get_cached_remote_package_path(&package);

        if package_path.exists() {
            return Ok(package_path);
        }

        // Create package_path dir
        Downloader::download_file(&package.dist.tarball, &package_path).await?;

        return Ok(package_path);
    }

    async fn get(&self, package: &RemotePackage) -> Option<PathBuf> {
        let package_path = self.get_cached_remote_package_path(&package);

        if !package_path.exists() {
            return None;
        }

        return Some(package_path);
    }
}
