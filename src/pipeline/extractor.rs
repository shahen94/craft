use std::path::PathBuf;
use std::sync::{mpsc::Sender, Arc};

use async_trait::async_trait;
use tokio::fs;
use tokio::sync::Mutex;

use super::artifacts::{ExtractArtifacts, StoredArtifact};
use crate::cache::DEP_CACHE_FOLDER;
use crate::fs::get_config_dir;
use crate::pipeline::ResolvedItem;
use crate::{
    contracts::{Phase, Pipe, PipeArtifact, ProgressAction},
    errors::{ExecutionError, ZipError},
    logger::CraftLogger,
    tar::Gzip,
};

pub struct ExtractorPipe {
    packages: Vec<StoredArtifact>,
    artifacts: Arc<Mutex<ExtractArtifacts>>,
    tmp_folder: PathBuf,
    tx: Sender<ProgressAction>,
}

impl ExtractorPipe {
    pub fn new(
        artifacts: &dyn PipeArtifact<Vec<StoredArtifact>>,
        tx: Sender<ProgressAction>,
    ) -> Self {
        let tmp_cache_folder = get_config_dir(DEP_CACHE_FOLDER.clone());

        Self {
            tmp_folder: tmp_cache_folder,
            packages: artifacts.get_artifacts(),
            artifacts: Arc::new(Mutex::new(ExtractArtifacts::new())),
            tx,
        }
    }

    // Skip because we now simlink the extracted files
    pub async fn cleanup(vec: Vec<ResolvedItem>) -> Result<(), ExecutionError> {
        use std::fs::metadata;

        let mapped_str = vec
            .iter()
            .map(|x| x.package.name.clone())
            .collect::<Vec<String>>();
        let mut entries = tokio::fs::read_dir("node_modules")
            .await
            .map_err(|e| ExecutionError::JobExecutionFailed(e.to_string(), e.to_string()))?;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| ExecutionError::JobExecutionFailed(e.to_string(), e.to_string()))?
        {
            let dir_name = entry.file_name().to_str().unwrap().to_string();
            let meta = metadata(entry.path())
                .map_err(|e| ExecutionError::JobExecutionFailed(e.to_string(), e.to_string()))?;
            if !mapped_str.contains(&dir_name) && meta.is_dir() {
                fs::remove_dir_all(entry.path()).await.map_err(|e| {
                    ExecutionError::JobExecutionFailed(e.to_string(), e.to_string())
                })?;
            } else if !mapped_str.contains(&dir_name) && meta.is_file() {
                fs::remove_file(entry.path()).await.map_err(|e| {
                    ExecutionError::JobExecutionFailed(e.to_string(), e.to_string())
                })?;
            }
        }

        Ok(())
    }

    pub async fn unzip_archive(&self, artifact: &StoredArtifact) -> Result<(), ZipError> {
        let artifact_s = artifact.clone();

        let tmp_folder = self.tmp_folder.clone();
        tokio::task::spawn_blocking(move || {
            let dest = tmp_folder.join(format!(
                "{}-{}",
                &artifact_s.package.name, &artifact_s.package.version
            ));

            // Skip if already unzipped
            if dest.exists() {
                return Ok(());
            }
            match Gzip::extract(&artifact_s.zip_path, &dest) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        })
        .await
        .unwrap()?;

        let extracted_at = self.tmp_folder.join(format!(
            "{}-{}",
            &artifact.package.name, &artifact.package.version
        ));

        self.artifacts
            .lock()
            .await
            .add(artifact.package.clone(), extracted_at);

        Ok(())
    }
}

#[async_trait]
impl Pipe<ExtractArtifacts> for ExtractorPipe {
    async fn run(&mut self) -> Result<ExtractArtifacts, ExecutionError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Extracting));

        for artifact in &self.packages {
            CraftLogger::verbose(format!("Extracting artifact: {}", artifact.package));
            self.unzip_archive(artifact).await.unwrap();
        }

        Ok(self.artifacts.lock().await.clone())
    }
}
