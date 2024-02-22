use std::sync::mpsc::Sender;

use async_trait::async_trait;

use crate::{
    contracts::{Logger, Phase, Pipe, ProgressAction},
    errors::ExecutionError,
    logger::CraftLogger,
    package::NpmPackage,
};

use super::artifacts::{ExtractArtifactsMap, LinkerArtifacts};

#[derive(Debug)]
pub struct LinkerPipe {
    tx: Sender<ProgressAction>,
    resolved: Vec<NpmPackage>,
    extracted: ExtractArtifactsMap,
    artifacts: LinkerArtifacts,
}

impl LinkerPipe {
    pub fn new(
        tx: Sender<ProgressAction>,
        resolved: Vec<NpmPackage>,
        extracted: ExtractArtifactsMap,
    ) -> Self {
        Self {
            tx,
            artifacts: LinkerArtifacts::default(),
            resolved,
            extracted,
        }
    }

    fn build_linker_artifacts(&mut self) {
        // We need to iterate over the resolved packages
        // And build the linker artifacts using extracted artifacts

        for package in &self.resolved {
            if let Some(extracted) = self.extracted.get(&package.name) {
                self.artifacts.add(extracted.unzip_at.clone());
            } else {
                CraftLogger::warn(format!(
                    "Package not found in extracted artifacts: {}@{}",
                    package.name, package.version
                ));
                todo!("Handle missing extracted artifacts");
            }
        }
    }

    fn link(&mut self) {
        println!("Linking artifacts: {:?}", self.artifacts.artifacts);
    }
}

#[async_trait]
impl Pipe<()> for LinkerPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Linking));

        self.build_linker_artifacts();
        self.link();

        Ok(())
    }
}
