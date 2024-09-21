use async_trait::async_trait;
use crate::cache::RegistryKey;
use crate::contracts::{Pipe};
use crate::errors::ExecutionError;
use crate::package::{BinType, PackageMetaRecorder, PackageRecorder, ResolvedBinary};

pub struct PeerResolver {
    recorder: PackageRecorder
}

impl PeerResolver {
    pub fn new(recorder: PackageRecorder) -> Self {
        PeerResolver {
            recorder
        }
    }

    fn handle_insert( req: &mut PackageMetaRecorder, outer_dep: (&RegistryKey, &PackageMetaRecorder)) {
        match &mut req.resolved_dependencies {
            Some(deps) => {
                deps.insert(outer_dep.0.name.clone(), outer_dep.0
                    .version.clone());
            }
            None => {
                let mut deps = std::collections::HashMap::new();
                deps.insert(outer_dep.0.name.clone(), outer_dep.0.version.clone());
                req.resolved_dependencies = Some(deps)
            }
        }
        if let Some(bin) = &outer_dep.1.bin {
            match &mut req.resolved_binaries {
                Some(b) => {
                    match bin {
                        BinType::Bin(s) => {
                            let splitted_name = s.rsplitn(2, '/').next().unwrap().replace(".js", "");
                            b.push(ResolvedBinary{
                                name: splitted_name,
                                path: format!("{}/{}", outer_dep.1.name, s),
                                package_name: outer_dep.1.name.clone()
                            });
                        }
                        BinType::BinMappings(a) => {
                            a.iter().for_each(|s| {
                                b.push(ResolvedBinary{
                                    name: s.0.to_string(),
                                    path: format!("{}/{}", outer_dep.1.name, s.1),
                                    package_name: outer_dep.1.name.clone()
                                });
                            });
                        }
                    }
                }
                None => {
                    match bin {
                        BinType::Bin(s) => {
                            let splitted_name = s.rsplitn(2, '/').next().unwrap().replace("\
                                .js", "");
                            let vec = vec![ResolvedBinary{
                                name: splitted_name,
                                path: format!("{}/{}", outer_dep.1.name, s),
                                package_name: outer_dep.1.name.clone()
                            }];
                            req.resolved_binaries = Some(vec);
                        }
                        BinType::BinMappings(a) => {
                            let mut bin_vec = vec![];
                            a.iter().for_each(|s| {
                                bin_vec.push(ResolvedBinary{
                                    name: s.0.to_string(),
                                    path: format!("{}/{}", outer_dep.1.name, s.1),
                                    package_name: outer_dep.1.name.clone()
                                });
                            });
                            req.resolved_binaries = Some(bin_vec);
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl Pipe<PackageRecorder> for PeerResolver {
    async fn run(&mut self) -> Result<PackageRecorder, ExecutionError> {
       self.recorder.sub_dependencies.clone().iter().for_each(|outer_dep|{
           if let Some(mut deps) = outer_dep.1.depth_traces.clone() {
               // Iterate over the traces
                deps.iter_mut().for_each(|d|{
                     d.reverse();
                    for (i, parent_of_outer_dep) in d.iter().enumerate() {
                        // This is a direct parent
                        if i == 0 {
                           if let Some(reg) = self.recorder.sub_dependencies.get_mut(parent_of_outer_dep) {
                               PeerResolver::handle_insert(reg, outer_dep);
                           } else if let Some(req) = self.recorder.main_packages.get_mut
                           (parent_of_outer_dep) {
                                 PeerResolver::handle_insert(req, outer_dep);
                           }
                        }
                    }
                })
           }
       });
        Ok(self.recorder.clone())
    }
}