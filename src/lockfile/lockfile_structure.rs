use crate::package::PackageMetaHandler;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::format;
use crate::lockfile::constants::{AUTO_INSTALL_PEERS, DEPENDENCIES, DEV_DEPENDENCIES, ENGINES, EXCLUDE_LINKS_FROM_LOCKFILE, LOCKFILE_VERSION, OPT_DEPENDENCIES, PEER_DEPENDENCIES, PEER_DEPENDENCIES_META, RESOLUTION, SETTINGS, SPECIFIER, VERSION};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResolvedDependency {
    pub specifier: String,
    pub version: String,
}

type ProjectId = String;
pub type ResolvedDependencies = HashMap<String, ResolvedDependency>;
type CatalogName = String;
type DependencyName = String;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockfileResolution {
    Directory(DirectoryResolution),
    GitRepository(GitRepositoryResolution),
    Tarball(TarballResolution),
    Integrity(IntegrityResolution),
}

/**
 * tarball hosted remotely
 */
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TarballResolution {
    pub r#type: Option<String>,
    pub tarball: Option<String>,
    pub integrity: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryResolution {
    directory: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRepositoryResolution {
    repo: String,
    commit: String,
    path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityResolution {
    pub integrity: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedCatalogEntry {
    pub specifier: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSnapshot {
    pub id: Option<String>,
    pub optional: Option<bool>,
    pub patched: Option<bool>,
    pub has_bin: Option<bool>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub resolution: LockfileResolution,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchFile {
    pub path: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchInfo {
    strict: bool,
    file: PatchFile,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LockfileSettings {
    pub auto_install_peers: Option<bool>,
    pub exclude_links_from_lockfile: Option<bool>,
    pub peers_suffix_max_length: Option<i32>,
}

fn default_lockfile_version() -> String {
    "9.0".to_string()
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImporterSections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<ResolvedDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<ResolvedDependencies>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockfileStructure {
    #[serde(default = "default_lockfile_version")]
    pub lockfile_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LockfileSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importers: Option<HashMap<ProjectId, ImporterSections>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<HashMap<CatalogName, HashMap<DependencyName, ResolvedCatalogEntry>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "ordered_map"
    )]
    pub packages: Option<HashMap<String, PackageMetaHandler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_built_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_built_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_extensions_checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_optional_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patched_dependencies: Option<HashMap<String, PatchFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnpmfile_checksum: Option<String>,
}


impl LockfileStructure {
    const ESCAPE_CHARS: [char; 3] = ['@', '<', '>', ];


    fn starts_with_illegal_character(str: &str) -> bool {
        if let Some(c) = str.chars().next() {
            if Self::ESCAPE_CHARS.contains(&c) {
                return true
            }
        }
        false
    }

    fn format_string(str: &str) -> String {
        if Self::starts_with_illegal_character(&str) {
            return format!("'{str}'")
        }
        str.to_string()
    }

    fn format_line(key: &str, value: Option<&str>, indent: i32) -> String {
        let mut str = "".to_string();
        for _ in 0..indent*2 {
            str.push_str(" ")
        }
        let key_format;
        let key = Self::format_string(key);
        match value {
            Some(v)=>{
                let v = Self::format_string(v);
                key_format = format!("{key}: {v}\n");
            }
            None=>{
                key_format = format!("{key}:\n");
            }
        }

        str.push_str(&key_format);
        str
    }

    fn format_settings(&self) -> String {
        let mut settings_str = "".to_string();
        let setting = self.settings.clone().unwrap();
        settings_str.push_str(&Self::format_line(SETTINGS, None,0));

        if let Some(a) = setting.auto_install_peers {
            let auto_install = Self::format_line(AUTO_INSTALL_PEERS, Some(&*a.to_string()), 1);
            settings_str.push_str(&*auto_install);
        }

        if let Some(e) = setting.exclude_links_from_lockfile {
            let exclude_link = Self::format_line(EXCLUDE_LINKS_FROM_LOCKFILE, Some(&*e.to_string()), 1);
            settings_str.push_str(&*exclude_link);
        }

        if let Some(peer) = setting.peers_suffix_max_length {
            let exclude_link = Self::format_line(EXCLUDE_LINKS_FROM_LOCKFILE, Some(&*peer.to_string()), 1);
            settings_str.push_str(&*exclude_link);
        }


        settings_str
    }

    fn format_lockfile_version(&self) -> String {
        Self::format_line(LOCKFILE_VERSION, Some(&format!("'{}'",self.lockfile_version)),0)
    }

    fn format_dependencies(title: &str, indent: i32, deps: &ResolvedDependencies) -> String {
        let mut dependency_serialized = "".to_string();
        dependency_serialized.push_str(&Self::format_line(title, None, indent));

        deps.iter().for_each(|d|{
           dependency_serialized.push_str(&Self::format_line(d.0, None, indent+1));
            dependency_serialized.push_str(&Self::format_line(SPECIFIER, Some(&d.1.specifier),indent+2));
            dependency_serialized.push_str(&Self::format_line(VERSION, Some(&d.1.version),indent+2));
        });


        dependency_serialized
    }

    fn format_importer(importer: (&ProjectId, &ImporterSections)) -> String {
        let mut importer_serialized = "\n".to_string();

        importer_serialized = Self::format_line(importer.0, None, 1);

        if let Some(dep) = &importer.1.dependencies {
            importer_serialized.push_str(&Self::format_dependencies(DEPENDENCIES, 2, dep))
        }

        if let Some(dev_deps) = &importer.1.dev_dependencies {
            importer_serialized.push_str(&Self::format_dependencies(DEV_DEPENDENCIES, 2, dev_deps))
        }

        if let Some(peer_deps) = &importer.1.peer_dependencies {
            importer_serialized.push_str(&Self::format_dependencies(PEER_DEPENDENCIES, 2, peer_deps))
        }

        if let Some(opt_deps) = &importer.1.optional_dependencies {
            importer_serialized.push_str(&Self::format_dependencies(OPT_DEPENDENCIES, 2, opt_deps))
        }


        importer_serialized

    }

    fn format_importers(&self) -> String {
        let mut importers_serialized = "".to_string();
        importers_serialized.push_str(&Self::format_line("importers", Some("\n"), 0));

        let importers = self.importers.clone().unwrap();


        importers.iter().for_each(|i|{
            let serialized_importer = Self::format_importer(i);
            importers_serialized.push_str(&serialized_importer);
        });


        importers_serialized
    }

    fn format_packages(&self) -> String {
        let mut packages_serialized = "packages:\n".to_string();

        let packages = self.packages.clone().unwrap();
        let packages: BTreeMap<_, _> = packages.iter().collect();
        let index = 1;
        packages.iter().for_each(|p|{
            packages_serialized.push_str("\n");
            packages_serialized.push_str(&Self::format_line(p.0, None, index));
            if let Some(res) = &p.1.resolution {
                let integrity = format!("{{integrity: {}}}", res.integrity);
                packages_serialized.push_str(&Self::format_line(RESOLUTION, Some(&integrity), index+1));
            }

            if let Some(peer) = &p.1.peer_dependencies {
                packages_serialized.push_str(&Self::format_line(PEER_DEPENDENCIES,None, index+1));
                peer.iter().for_each(|(k,v)| {
                    packages_serialized.push_str(&Self::format_line(k,Some(v), index+2));
                })
            }

            if let Some(peer_meta) = &p.1.peer_dependencies_meta {
                packages_serialized.push_str(&Self::format_line(PEER_DEPENDENCIES_META,None, index+1));
                peer_meta.iter().for_each(|(k,v)| {
                    packages_serialized.push_str(&Self::format_line(k,None, index+2));
                })
            }


            if let Some(engines) = &p.1.engines {
                match engines.len() == 1 {
                    true=>{
                        let engine_val = engines.iter().next().unwrap();
                        let node = format!("{{{}: {}}}",engine_val.0, Self::format_string(engine_val.1));
                        packages_serialized.push_str(&Self::format_line(ENGINES, Some(&node), index+1));
                    },
                    false =>{
                        packages_serialized.push_str(&Self::format_line(ENGINES, None, index+1));

                        engines.iter().for_each(|e|{
                            packages_serialized.push_str(&Self::format_line(e.0, Some(e.1), index+2))
                        })
                    }
                }
            }
        });



        packages_serialized
    }

    pub fn write_to_string(&self) -> String {
        let mut serialized_content = "".to_string();
        serialized_content.push_str(&self.format_lockfile_version());


        if self.settings.is_some() {
            serialized_content.push_str("\n");
            serialized_content.push_str(&self.format_settings())
        }

        if self.importers.is_some() {
            serialized_content.push_str("\n");
            serialized_content.push_str(&self.format_importers())
        }

        if self.packages.is_some() {
            serialized_content.push_str("\n");
            serialized_content.push_str(&self.format_packages())
        }

        serialized_content
    }
}



fn ordered_map<S>(
    value: &Option<HashMap<String, PackageMetaHandler>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    return match value {
        None => return serializer.serialize_none(),
        Some(v) => {
            let ordered: BTreeMap<_, _> = v.iter().collect();
            ordered.serialize(serializer)
        }
    };
}

impl Default for LockfileStructure {
    fn default() -> Self {
        let default_settings = LockfileSettings{
            auto_install_peers: Some(true),
            exclude_links_from_lockfile: Some(false),
            peers_suffix_max_length: None
        };


        LockfileStructure {
            lockfile_version: "9.0".to_string(),
            importers: None,
            ignored_optional_dependencies: None,
            overrides: None,
            catalogs: None,
            only_built_dependencies: None,
            package_extensions_checksum: None,
            settings: Some(default_settings),
            time: None,
            patched_dependencies: None,
            pnpmfile_checksum: None,
            never_built_dependencies: None,
            packages: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectSnapshot {
    specifiers: ResolvedDependencies,
    dependencies: Option<ResolvedDependencies>,
    optional_dependencies: Option<ResolvedDependencies>,
    dev_dependencies: Option<ResolvedDependencies>,
    dependencies_meta: Option<DependenciesMeta>,
}

pub type DependenciesMeta = HashMap<DependencyName, DependencyMeta>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyMeta {
    injected: Option<bool>,
    node: Option<String>,
    patch: Option<String>,
}
