use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use async_trait::async_trait;

use crate::actors::peer_resolver::PeerResolver;
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

#[derive(Debug, Clone)]
pub enum PackageType {
    Dev(String),
    Optional(String),
    Prod(String),
    Peer(String),
    Global(String),
}

impl PackageType {
    pub fn get_parts(&self) -> (String, String) {
        fn split_name(key: &str) -> (String, String) {
            let nums_of_ats = key.chars().filter(|c| *c == '@').count();
            let is_scoped_package = key.starts_with("@");

            // Normal case like is-even@1
            if !is_scoped_package && nums_of_ats == 1 {
                let parts = key.rsplitn(2, '@').collect::<Vec<_>>();
                return (parts[1].to_string(), parts[0].to_string());
            }

            if is_scoped_package && nums_of_ats == 2 {
                let parts = key.rsplitn(2, '@').collect::<Vec<_>>();
                return (parts[1].to_string(), parts[0].to_string());
            }

            // Normal case like is-even => no version
            if nums_of_ats == 0 {
                return (key.to_string(), "*".to_string());
            }

            // Normal case like @babel/transform@1
            if is_scoped_package && nums_of_ats == 1 {
                // There is no version available
                return (key.to_string(), "*".to_string());
            }

            // We can't have like 3 @s
            panic!("Too many @s")
        }
        match self {
            PackageType::Dev(d) => split_name(d),
            PackageType::Optional(o) => split_name(o),
            PackageType::Prod(p) => split_name(p),
            PackageType::Peer(peer) => split_name(peer),
            PackageType::Global(g) => split_name(g),
        }
    }
}

pub struct InstallActor {
    packages: Vec<PackageType>,
}

impl InstallActor {
    pub fn new(packages: Vec<PackageType>) -> Self {
        Self { packages }
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
        CraftLogger::verbose(format!(
            "Resolved: {:?}",
            resolve_artifacts.0.get_artifacts().len()
        ));

        // ─── Start Mutating ───────────────────────
        let recorder = PeerResolver::new(resolve_artifacts.1).run().await?;

        // ─── Start Downloading ──────────────────────

        CraftLogger::verbose("Downloading dependencies");
        let download_artifacts = DownloaderPipe::new(&resolve_artifacts.0, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose(format!(
            "Downloaded {:?}",
            download_artifacts.get_artifacts().len()
        ));

        // ─── Start Extracting ───────────────────────

        CraftLogger::verbose("Extracting dependencies");
        let extracted_artifacts = ExtractorPipe::new(&download_artifacts, tx.clone())
            .run()
            .await?;

        CraftLogger::verbose(format!(
            "Extracted {:?}",
            extracted_artifacts.get_artifacts().len()
        ));

        // ─── Start Linking ──────────────────────────

        CraftLogger::verbose("Linking dependencies");
        LinkerPipe::new(
            tx.clone(),
            resolve_artifacts.0.get_artifacts(),
            extracted_artifacts.get_artifacts(),
            recorder.clone(),
        )
        .run()
        .await?;

        // ─── Sync Lock File ────────────────────────
        LockFileActor::new(resolve_artifacts.0.get_artifacts(), recorder)
            .run()
            .expect("Error writing lockfile");

        // ─── Cleanup ────────────────────────────────

        ExtractorPipe::cleanup(resolve_artifacts.0.get_artifacts()).await?;

        // ─── Link binaries ────────────────────────────────

        drop(tx);
        ui_thread.join().unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actors::PackageType;
    use std::collections::HashMap;

    #[test]
    pub fn test_get_parts() {
        let mut mappings = HashMap::new();
        mappings.insert("@lodash/test@1", ("@lodash/test", "1"));
        mappings.insert("@lodash/test", ("@lodash/test", "*"));
        mappings.insert("is-even@~1", ("is-even", "~1"));
        mappings.insert("is-even@~1.2.0", ("is-even", "~1.2.0"));
        mappings.insert("is-even", ("is-even", "*"));

        mappings.iter().for_each(|(k, v)| {
            let pkg_type = PackageType::Dev(k.to_string());
            let parts = pkg_type.get_parts();

            assert_eq!(parts.0, v.0);
            assert_eq!(parts.1, v.1);
        })
    }
}
