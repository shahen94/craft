use crate::actors::PackageType;
use crate::contracts::Lockfile;
use crate::errors::LockfileError;
use crate::lockfile::constants::CURRENT_IMPORTER;
use crate::lockfile::lockfile_structure::{
    ImporterSections, LockfileStructure, ResolvedDependency,
};
use crate::package::{PackageMetaHandler, PackageRecorder};
use crate::pipeline::ResolvedItem;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct LockFileActor {
    resolved_items: Vec<ResolvedItem>,
    recorder: PackageRecorder,
}

impl LockFileActor {
    pub(crate) fn new(
        resolved_items: Vec<ResolvedItem>,
        recorder: PackageRecorder,
    ) -> LockFileActor {
        LockFileActor {
            resolved_items,
            recorder,
        }
    }

    fn persist_lockfile_structure(
        lockfile_structure: LockfileStructure,
    ) -> Result<(), LockfileError> {
        let string = serde_yaml::to_string(&lockfile_structure).unwrap();
        fs::write("pnpm-lock.yaml", string)
            .map_err(|e| LockfileError::FileWriteError(e.to_string()))?;
        Ok(())
    }

    fn create_importers(
        packages: Vec<ResolvedItem>,
        map: Option<&ImporterSections>,
    ) -> ImporterSections {
        let mut map_to_use = map.cloned().unwrap_or_default();

        packages.iter().for_each(|item| {
            if item.parent.is_none() {
                match item.package_type {
                    PackageType::Dev(_) => match &mut map_to_use.dev_dependencies {
                        Some(ref mut dev_d) => {
                            dev_d.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                        }
                        None => {
                            let mut dev_deps = HashMap::new();
                            dev_deps.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                            map_to_use.dev_dependencies = Some(dev_deps)
                        }
                    },
                    PackageType::Optional(_) => match map_to_use.optional_dependencies {
                        Some(ref mut opt_d) => {
                            opt_d.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                        }
                        None => {
                            let mut dev_deps = HashMap::new();
                            dev_deps.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                            map_to_use.optional_dependencies = Some(dev_deps)
                        }
                    },
                    PackageType::Prod(_) => match map_to_use.dependencies {
                        Some(ref mut opt_d) => {
                            opt_d.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                        }
                        None => {
                            let mut prod_deps = HashMap::new();
                            prod_deps.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                            map_to_use.dependencies = Some(prod_deps)
                        }
                    },
                    PackageType::Peer(_) => match map_to_use.peer_dependencies {
                        Some(ref mut opt_d) => {
                            opt_d.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                        }
                        None => {
                            let mut prod_deps = HashMap::new();
                            prod_deps.insert(
                                item.package.name.clone(),
                                ResolvedDependency {
                                    version: item.package.version.clone(),
                                    specifier: item.specifier.clone(),
                                },
                            );
                            map_to_use.peer_dependencies = Some(prod_deps)
                        }
                    },
                    PackageType::Global(_) => {}
                }
            }
        });
        map_to_use
    }

    fn handle_importers(
        &self,
        lockfile_structure: &mut LockfileStructure,
    ) -> Result<(), LockfileError> {
        match &mut lockfile_structure.importers {
            Some(e) => {
                let current_importer = e.get(CURRENT_IMPORTER);
                match current_importer {
                    Some(i) => {
                        e.insert(
                            CURRENT_IMPORTER.to_string(),
                            Self::create_importers(self.resolved_items.clone(), Some(&i.clone())),
                        );
                        Ok(())
                    }
                    None => {
                        let resolved_deps =
                            Self::create_importers(self.resolved_items.clone(), None);
                        e.insert(CURRENT_IMPORTER.to_string(), resolved_deps);
                        Ok(())
                    }
                }
            }
            None => {
                let mut new_importers = HashMap::new();
                new_importers.insert(
                    CURRENT_IMPORTER.to_string(),
                    Self::create_importers(self.resolved_items.clone(), None),
                );

                lockfile_structure.importers = Some(new_importers);
                Ok(())
            }
        }
    }

    fn handle_packages(&self, lockfile_structure: &mut LockfileStructure) {
        let mut hashmap: HashMap<String, PackageMetaHandler> = HashMap::new();

        self.recorder.main_packages.iter().for_each(|p| {
            let pm_handler: PackageMetaHandler = p.clone().into();
            hashmap.insert(p.name.clone(), pm_handler);
        });

        self.recorder.sub_dependencies.iter().for_each(|p| {
            let pm_handler: PackageMetaHandler = p.clone().into();
            hashmap.insert(p.name.clone(), pm_handler);
        });

        lockfile_structure.packages = Some(hashmap)
    }
}

impl Lockfile<LockfileStructure> for LockFileActor {
    fn read_lock_file(path: &Path) -> Result<LockfileStructure, LockfileError> {
        let file =
            fs::read_to_string(path).map_err(|e| LockfileError::FileReadError(e.to_string()))?;
        let structure = serde_yaml::from_str::<LockfileStructure>(&file)
            .map_err(|e| LockfileError::InvalidStructure(e.to_string()))?;
        Ok(structure)
    }

    fn run(&self) -> Result<(), LockfileError> {
        if fs::exists("pnpm-lock.yaml").expect("Can't check existence of file does_not_exist.txt") {
            let mut lockfile_structure = Self::read_lock_file(Path::new("pnpm-lock.yaml"))?;
            self.handle_importers(&mut lockfile_structure)?;
            self.handle_packages(&mut lockfile_structure);
            Self::persist_lockfile_structure(lockfile_structure)?;
            Ok(())
        } else {
            let mut lockfile_structure = LockfileStructure::default();
            self.handle_importers(&mut lockfile_structure)?;
            Self::persist_lockfile_structure(lockfile_structure)?;
            Ok(())
        }
    }
}
