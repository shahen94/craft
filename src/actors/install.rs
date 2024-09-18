use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use async_trait::async_trait;

use crate::cache::PackagesCache;
use crate::contracts::{Lockfile, PersistentCache};
use crate::lockfile::lock_file_actor::LockFileActor;
use crate::{
    contracts::{Actor, Pipe, PipeArtifact, Progress, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    pipeline::{DownloaderPipe, ExtractorPipe, LinkerPipe, ResolverPipe},
    ui::UIProgress,
};
use crate::command::Install;

pub struct InstallActor {
    packages: Vec<String>,
    install: Option<Install>
}

impl InstallActor {
    pub fn new(packages: Vec<String>, install: Option<Install>) -> Self {
        Self { packages, install }
    }

    fn start_progress(&self, rx: Receiver<ProgressAction>) -> JoinHandle<()> {
        thread::spawn(move || {
            let progress = UIProgress::default();

            progress.start(rx);
        })
    }
}

pub(crate) type PipeResult = Result<(), ExecutionError>;

#[async_trait]
impl Actor<PipeResult> for InstallActor {
    async fn start(&mut self) -> PipeResult {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut cache = PackagesCache::default();
        cache.init().await.unwrap();
        let ui_thread = self.start_progress(rx);

        // ─── Start Resolving ─────────────────────────

        CraftLogger::verbose("Resolving dependencies");
        let resolve_artifacts = ResolverPipe::new(self.packages.clone(), tx.clone())
            .run()
            .await?;
        CraftLogger::verbose(
            format!("Resolved: {:?}", resolve_artifacts.get_artifacts().len()),
        );

        // ─── Start Downloading ──────────────────────

        CraftLogger::verbose("Downloading dependencies");
        let download_artifacts = DownloaderPipe::new(&resolve_artifacts, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose(
            format!("Downloaded {:?}", download_artifacts.get_artifacts().len()),
        );

        // ─── Start Extracting ───────────────────────

        CraftLogger::verbose("Extracting dependencies");
        let extracted_artifacts = ExtractorPipe::new(&download_artifacts, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose(
            format!("Extracted {:?}", extracted_artifacts.get_artifacts().len()),
        );

        // ─── Start Linking ──────────────────────────

        CraftLogger::verbose("Linking dependencies");
        LinkerPipe::new(
            tx.clone(),
            resolve_artifacts.get_artifacts(),
            extracted_artifacts.get_artifacts(),
        )
        .run()
        .await?;

        // ─── Sync Lock File ────────────────────────
        LockFileActor::new(resolve_artifacts.get_artifacts(), self.install.clone())
            .run()
            .unwrap();

        // ─── Cleanup ────────────────────────────────

        ExtractorPipe::cleanup(resolve_artifacts.get_artifacts()).await?;

        drop(tx);
        ui_thread.join().unwrap();
        Ok(())
    }
}
