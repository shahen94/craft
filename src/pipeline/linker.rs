use std::{env, fs, path::PathBuf, sync::mpsc::Sender};

use async_trait::async_trait;
use lazy_static::lazy_static;

use super::artifacts::{ExtractArtifactsMap, LinkArtifactItem, ResolvedItem};
use crate::{
    contracts::{Logger, Phase, Pipe, ProgressAction},
    errors::ExecutionError,
    fs::copy_dir,
    logger::CraftLogger,
};
use path_clean::clean;

use crate::package::{BinType, PackageRecorder, ResolvedBinary};
use crate::pipeline::binary_templates::{get_bash_script, get_cmd_script, get_pwsh_script};
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct LinkerPipe {
    tx: Sender<ProgressAction>,
    resolved: Vec<ResolvedItem>,
    extracted: ExtractArtifactsMap,
    recorder: PackageRecorder,
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
        recorder: PackageRecorder,
    ) -> Self {
        Self {
            tx,
            resolved,
            extracted,
            recorder,
        }
    }

    fn build_linker_artifacts(&mut self) -> Vec<LinkArtifactItem> {
        let mut linker_artifacts = vec![];

        // So that the parents (things in our package.json come first)
        self.resolved.reverse();

        for resolved in &self.resolved {
            let pkg = &resolved.package;
            let parent = &resolved.parent;

            if resolved.package.name == "body-parser" {
                CraftLogger::info(format!("Parent: {:?}", parent));
            }

            let from = self.extracted.get(&pkg.to_string()).unwrap().clone();

            // If it is a child
            let to = if let Some(path_vec) = parent {
                let mut path = PathBuf::new();

                for p in path_vec {
                    path.push(&p.name);
                    path.push("node_modules")
                }
                NODE_MODULES.join(&path).join(&pkg.name)
            } else {
                NODE_MODULES.join(&pkg.name)
            };

            linker_artifacts.push(LinkArtifactItem::new(from.unzip_at, to));
        }

        linker_artifacts
    }

    async fn link(&mut self, artifacts: &Vec<LinkArtifactItem>) {
        for artifact in artifacts {
            if let Err(e) = fs::create_dir_all(&artifact.to) {
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

    fn prepare_bin_dir(bin_dir_to_create: &PathBuf, rb: &ResolvedBinary) {
        if fs::metadata(bin_dir_to_create).is_err() {
            let result = fs::create_dir(bin_dir_to_create);
            if let Err(e) = result {
                log::error!(
                    "Failed to create directory: {}",
                    bin_dir_to_create.display()
                );
                log::error!("Error: {}", e);
                return;
            }
        }
        if fs::metadata(bin_dir_to_create.join(&rb.name)).is_err() {
            let mut abs_path = clean(bin_dir_to_create.join(".."));

            if !abs_path.is_absolute() {
                abs_path = env::current_dir().unwrap().join(abs_path);
            }

            let result = fs::write(
                bin_dir_to_create.join(&rb.name),
                get_bash_script(
                    vec![abs_path.display().to_string()],
                    &rb.package_name,
                    &rb.path,
                ),
            );
            if let Err(e) = result {
                log::error!("Failed to write file: {}", bin_dir_to_create.display());
                log::error!("Error: {}", e);
                return;
            }
        }

        if fs::metadata(bin_dir_to_create.join(format!("{}.CMD", rb.name))).is_err() {
            let mut abs_path = clean(bin_dir_to_create.join(".."));

            if !abs_path.is_absolute() {
                abs_path = env::current_dir().unwrap().join(abs_path);
            }

            let result = fs::write(
                bin_dir_to_create.join(format!("{}.CMD", rb.name)),
                get_cmd_script(
                    vec![abs_path.display().to_string()],
                    &rb.package_name,
                    &rb.path,
                ),
            );
            if let Err(e) = result {
                log::error!("Failed to write file: {}", bin_dir_to_create.display());
                log::error!("Error: {}", e);
                return;
            }
        }

        if fs::metadata(bin_dir_to_create.join(format!("{}.ps1", rb.name))).is_err() {
            let mut abs_path = clean(bin_dir_to_create.join(".."));

            if !abs_path.is_absolute() {
                abs_path = env::current_dir().unwrap().join(abs_path);
            }

            let result = fs::write(
                bin_dir_to_create.join(format!("{}.ps1", rb.name)),
                get_pwsh_script(
                    vec![abs_path.display().to_string()],
                    &rb.package_name,
                    &rb.path,
                ),
            );
            if let Err(e) = result {
                log::error!("Failed to write file: {}", bin_dir_to_create.display());
                log::error!("Error: {}", e);
            }
        }
    }

    async fn link_binaries(&self) {
        self.recorder.main_packages.iter().for_each(|p| {
            if let Some(r_opt) = &p.1.resolved_binaries {
                let path_to_bin =
                    p.1.resolve_path_to_package()
                        .join("node_modules")
                        .join(".bin");
                for r in r_opt {
                    log::info!("{:?}", p.1.resolve_path_to_package());
                    Self::prepare_bin_dir(&path_to_bin, r);
                }
            }

            if let Some(bin) = &p.1.bin {
                match bin {
                    BinType::Bin(s) => {
                        let path_to_bin = PathBuf::from("node_modules").join(".bin");
                        let resolved_binary = ResolvedBinary {
                            name: s.rsplit('/').next().unwrap().replace(".js", ""),
                            path: s.clone(),
                            package_name: p.1.name.clone(),
                        };
                        Self::prepare_bin_dir(&path_to_bin, &resolved_binary);
                    }
                    BinType::BinMappings(a) => {
                        a.iter().for_each(|s| {
                            let resolved_binary = ResolvedBinary {
                                name: s.0.to_string(),
                                path: s.1.clone(),
                                package_name: p.1.name.clone(),
                            };
                            let path_to_bin = PathBuf::from("node_modules").join(".bin");
                            Self::prepare_bin_dir(&path_to_bin, &resolved_binary);
                        });
                    }
                }
            }
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[async_trait]
impl Pipe<()> for LinkerPipe {
    async fn run(&mut self) -> Result<(), ExecutionError> {
        let _ = self.tx.send(ProgressAction::new(Phase::Linking));

        let artifacts = self.build_linker_artifacts();

        self.link(&artifacts).await;
        self.link_binaries().await;

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
