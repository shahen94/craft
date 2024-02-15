use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{contracts::LOCK_FILE_NAME, package::NpmPackage};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LockFile {
    packages: Vec<NpmPackage>,
}

impl LockFile {
    #[allow(dead_code)]
    pub async fn load(path: &Path) -> Self {
        let file_path = path.join(LOCK_FILE_NAME);

        if !file_path.exists() {
            return LockFile::default();
        }

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();

        let lock_file: LockFile = serde_json::from_str(&content).unwrap();

        lock_file
    }

    pub async fn sync(package: Vec<NpmPackage>, path: PathBuf) {
        let file_path = path.join(LOCK_FILE_NAME);

        if file_path.exists() {
            tokio::fs::remove_file(&file_path).await.unwrap();
        }

        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        let content = serde_json::to_string(&LockFile { packages: package }).unwrap();

        tokio::io::copy(&mut content.as_bytes(), &mut file)
            .await
            .unwrap();
    }
}
