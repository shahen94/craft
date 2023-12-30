use std::{fs, io::Write, path::PathBuf};

use async_trait::async_trait;

use crate::common::{
    contracts::Modules,
    errors::{GzipDownloadError, UninstallError, UnzipError},
    remote_package::RemotePackage,
};

use flate2::read::GzDecoder;
use tar::Archive;

const TEMPORARY_FOLDER: &str = ".craft";

#[derive(Debug)]
pub struct NodeModules {
    pub path: PathBuf,
}

impl NodeModules {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        Self { path }
    }

    pub fn init_folder(&self) {
        // We should create node_modules if it doesn't exists, and .craft inside of it if it doesn't exists

        // We should create node_modules/.craft if it doesn't exists
        let craft_path = self.path.join(TEMPORARY_FOLDER);
        if !craft_path.exists() {
            std::fs::create_dir_all(craft_path).unwrap();
        }
    }

    pub fn get_gzip_name(&self, package: &RemotePackage) -> String {
        format!("{}-{}.tgz", package.name, package.version)
    }

    pub fn cleanup_package_temporary_data(
        &self,
        archive_path: &PathBuf,
        unzip_folder: &PathBuf,
    ) -> Result<(), UnzipError> {
        fs::remove_file(&archive_path)?;
        fs::remove_dir(&unzip_folder)?;

        Ok(())
    }
}

#[async_trait]
impl Modules for NodeModules {
    async fn download_package(
        &self,
        package: &RemotePackage,
    ) -> Result<PathBuf, GzipDownloadError> {
        let url = package.dist.tarball.clone();
        let name = self.get_gzip_name(&package);
        let path = self.path.join(TEMPORARY_FOLDER).join(name.clone());

        if path.exists() {
            return Ok(PathBuf::from(name));
        }

        let response = reqwest::get(&url).await?;
        let mut file = fs::File::create(path.clone()).map_err(|err| {
            let error_msg = format!("Error creating file: {:?}", err);
            GzipDownloadError::new(error_msg)
        })?;

        if !response.status().is_success() {
            let error_msg = format!("Package {}@{} not found", package.name, package.version);
            return Err(GzipDownloadError::new(error_msg));
        }

        let mut content = response.bytes().await?;

        match file.write_all(&mut content) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error writing file: {:?}", error);
                return Err(GzipDownloadError::new(error_msg));
            }
        };

        return Ok(PathBuf::from(name));
    }

    async fn unzip_package(&self, package: &RemotePackage) -> Result<(), UnzipError> {
        let name = self.get_gzip_name(&package);
        let archive_path = self.path.join(TEMPORARY_FOLDER).join(name.clone());
        // We should unzip file in the folder ./node_modules/.craft/gzip-filename/
        // After unzip we'll receive folder package
        // we should move this to the ./node_modules/{package_name}

        if !archive_path.exists() {
            let error_msg = format!("Package {}@{} not found", package.name, package.version);
            return Err(UnzipError::new(error_msg));
        }

        let unzip_folder = self.path.join(TEMPORARY_FOLDER).join(package.name.clone());
        let final_path = self.path.join(package.name.clone());

        if final_path.exists() {
            self.cleanup_package_temporary_data(&archive_path, &unzip_folder)?;
            return Ok(());
        }

        let file = fs::File::open(archive_path.clone()).map_err(|err| {
            let error_msg = format!("Error opening file: {:?}", err);
            UnzipError::new(error_msg)
        })?;

        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);

        match archive.unpack(&unzip_folder) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error unpacking file: {:?}", error);
                return Err(UnzipError::new(error_msg));
            }
        };

        match fs::rename(unzip_folder.join("package"), final_path) {
            Ok(_) => {}
            Err(error) => {
                let error_msg = format!("Error renaming folder: {:?}", error);
                return Err(UnzipError::new(error_msg));
            }
        };

        self.cleanup_package_temporary_data(&archive_path, &unzip_folder)?;

        return Ok(());
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
