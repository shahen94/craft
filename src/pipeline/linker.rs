use std::{env, path::PathBuf, sync::mpsc::Sender};

use async_trait::async_trait;
use lazy_static::lazy_static;

use crate::{
    contracts::{Logger, Phase, Pipe, ProgressAction},
    errors::ExecutionError,
    fs::copy_dir,
    logger::CraftLogger,
};

use super::artifacts::{ExtractArtifactsMap, LinkArtifactItem, ResolvedItem};

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct LinkerPipe {
    tx: Sender<ProgressAction>,
    resolved: Vec<ResolvedItem>,
    extracted: ExtractArtifactsMap,
}

// ─────────────────────────────────────────────────────────────────────────────

lazy_static! {
    pub static ref NODE_MODULES: PathBuf = env::current_dir().unwrap().join("node_modules");
}

// ─────────────────────────────────────────────────────────────────────────────

impl LinkerPipe {
    pub fn new(
        tx: Sender<ProgressAction>,
        resolved: Vec<ResolvedItem>,
        extracted: ExtractArtifactsMap,
    ) -> Self {
        Self {
            tx,
            resolved,
            extracted,
        }
    }

    fn build_linker_artifacts(&mut self) -> Vec<LinkArtifactItem> {
        let mut linker_artifacts = vec![];

        for resolved in &self.resolved {
            let pkg = &resolved.package;
            let parent = &resolved.parent;

            if resolved.package.name == "body-parser" {
                CraftLogger::info(format!("Parent: {:?}", parent));
            }

            let from = self.extracted.get(&pkg.to_string()).unwrap().clone();
            let to = NODE_MODULES.join(&pkg.name);
            let to = if let Some(parent) = parent {
                let path_vec = parent.split("/").collect::<Vec<&str>>();
                let mut path = PathBuf::new();

                for p in path_vec {
                    path.push(p);
                }
                NODE_MODULES
                    .join(&path)
                    .join("node_modules")
                    .join(&pkg.name)
            } else {
                NODE_MODULES.join(&pkg.name)
            };

            linker_artifacts.push(LinkArtifactItem::new(from.unzip_at, to));
        }

        linker_artifacts
    }

    async fn link(&mut self, artifacts: Vec<LinkArtifactItem>) {
        for artifact in artifacts {
            if let Err(e) = std::fs::create_dir_all(&artifact.to) {
                CraftLogger::error(format!(
                    "Failed to create directory: {}",
                    artifact.to.display()
                ));
                CraftLogger::error(format!("Error: {}", e));
            }

            if let Err(e) = copy_dir(&artifact.from.join("package"), &artifact.to) {
                CraftLogger::error(format!(
                    "Failed to copy from: {} to: {}: Error: {}",
                    artifact.from.display(),
                    artifact.to.display(),
                    e
                ));
                CraftLogger::error(format!("Error: {}", e));
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl Pipe<()> for LinkerPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Linking));

        let artifacts = self.build_linker_artifacts();

        self.link(artifacts).await;

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
