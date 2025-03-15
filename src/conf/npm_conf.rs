#[allow(clippy::all)]
use crate::conf::constants::*;
use crate::errors::ExecutionError;
use crate::pipeline::{determine_global_config_file_location, parse_config};
use chrono::NaiveDate;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::string::ToString;

#[derive(Debug)]
pub enum Access {
    Public,
    Restricted,
    Null,
}

#[derive(Debug)]
pub enum AuthType {
    Web,
    Legacy,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Depth {
    StringVal(String),
    IntVal(i32),
}

#[derive(Debug)]
pub enum Include {
    Dev,
    Optional,
    Prod,
    Peer,
}

#[derive(Debug)]
pub struct NpmConfig {
    pub _auth: Option<String>,
    pub access: Access,
    pub all: bool,
    pub allow_same_version: bool,
    pub audit: bool,
    pub audit_level: Option<String>,
    pub auth_type: AuthType,
    pub before: Option<NaiveDate>,
    pub bin_links: bool,
    pub browser: String,
    pub ca: Option<String>,
    pub cache: String,
    pub ca_file: Option<String>,
    pub call: Option<String>,
    pub cidr: Option<String>,
    pub color: bool,
    pub commit_hooks: bool,
    pub cpu: Option<String>,
    pub depth: Depth,
    pub description: bool,
    pub diff: String,
    pub diff_dst_prefix: String,
    pub diff_ignore_all_space: bool,
    pub diff_name_only: bool,
    pub diff_no_prefix: bool,
    pub diff_src_prefix: String,
    pub diff_text: bool,
    pub diff_unified: i32,
    pub dry_run: bool,
    pub editor: String,
    pub engine_strict: bool,
    pub expect_result_count: Option<i32>,
    pub expect_results: Option<bool>,
    pub fetch_retries: i32,
    pub fetch_retry_factor: i32,
    pub fetch_retry_maxtimeout: i32,
    pub fetch_retry_mintimeout: i32,
    pub fetch_timeout: i32,
    pub force: bool,
    pub foreground_scripts: bool,
    pub format_package_lock: bool,
    pub fund: bool,
    pub git: String,
    pub git_tag_version: bool,
    pub global: bool,
    pub globalconfig: Option<String>,
    pub heading: String,
    pub https_proxy: Option<String>,
    pub if_present: bool,
    pub ignore_scripts: bool,
    pub include: Option<Include>,
    pub include_staged: bool,
    pub include_workspace_root: bool,
    pub init_author_email: Option<String>,
    pub init_author_name: Option<String>,
    pub init_author_url: Option<String>,
    pub init_license: String,
    pub init_module: String,
    pub init_version: String,
    pub install_links: bool,
    pub install_strategy: InstallStrategy,
    pub json: bool,
    pub legacy_peer_deps: bool,
    pub libc: Option<String>,
    pub link: bool,
    pub local_address: Option<String>,
    pub _location: Location,
    pub lockfile_version: i32,
    pub _log_level: LogLevel,
    pub logs_dir: String,
    pub logs_max: i32,
    pub long: bool,
    pub max_sockets: i32,
    pub message: String,
    pub node_options: Option<String>,
    pub no_proxy: Option<String>,
    pub offline: bool,
    pub omit: Option<String>,
    pub omit_lockfile_registry_resolved: bool,
    pub os: Option<String>,
    pub otp: Option<String>,
    pub pack_destination: Option<String>,
    pub package: String,
    pub package_lock: bool,
    pub package_lock_only: bool,
    pub parseable: bool,
    pub prefer_dedupe: bool,
    pub prefer_offline: bool,
    pub prefer_online: bool,
    pub prefix: String,
    pub preid: Option<String>,
    pub progress: bool,
    pub provenance: bool,
    pub provenance_file: Option<String>,
    pub proxy: Option<String>,
    pub read_only: bool,
    pub rebuild_bundle: bool,
    pub registry: String,
    pub replace_registry_host: String,
    pub save: bool,
    pub save_bundle: bool,
    pub save_dev: bool,
    pub save_exact: bool,
    pub save_optional: bool,
    pub save_peer: bool,
    pub save_prefix: String,
    pub save_prod: bool,
    pub sbom_format: Option<String>,
    pub sbom_type: String,
    pub scope: Option<String>,
    pub script_shell: String,
    pub search_exclude: Option<String>,
    pub search_limit: i32,
    pub search_opts: Option<String>,
    pub search_staleness: i32,
    pub shell: String,
    pub sign_git_commit: bool,
    pub sign_git_tag: bool,
    pub strict_peer_deps: bool,
    pub strict_ssl: bool,
    pub tag: String,
    pub tag_version_prefix: String,
    pub timing: bool,
    pub umask: i32,
    pub unicode: bool,
    pub update_notifier: bool,
    pub usage: bool,
    pub user_agent: String,
    pub user_config: String,
    pub version: bool,
    pub versions: bool,
    pub viewer: String,
    pub which: Option<i32>,
    pub workspace: Option<String>,
    pub workspaces: Option<bool>,
    pub workspaces_update: bool,
    pub yes: Option<bool>,
    pub also: Option<String>,
    pub cache_max: i32,
    pub cache_min: i32,
    pub cert: Option<String>,
    pub _key: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum Location {
    User,
    Global,
    Project,
}

#[derive(Debug)]
pub enum InstallStrategy {
    Hoisted,
    Nested,
    Shallow,
    Linked,
}

#[derive(Debug)]
pub enum LogLevel {
    Silent,
    Error,
    Warn,
    Notice,
    Verbose,
    Http,
    Timing,
    Info,
    Https,
    Silly,
    None,
}

fn create_omit() -> Option<String> {
    if let Ok(e) = std::env::var("NODE_ENV") {
        if e == "production" {
            return Some("dev".parse().unwrap());
        }
    }

    None
}

fn create_no_proxy() -> Option<String> {
    if let Ok(e) = std::env::var("NO_PROXY") {
        return Some(e);
    }

    if let Ok(e) = std::env::var("no_proxy") {
        return Some(e);
    }

    None
}

fn create_node_options() -> Option<String> {
    if let Ok(e) = std::env::var("NODE_OPTIONS") {
        return Some(e);
    }

    None
}

fn create_https_proxy_conf() -> Option<String> {
    if let Ok(e) = std::env::var("HTTPS_PROXY") {
        return Some(e);
    }

    if let Ok(e) = std::env::var("https_proxy") {
        return Some(e);
    }

    if let Ok(e) = std::env::var("HTTP_PROXY") {
        return Some(e);
    }

    if let Ok(e) = std::env::var("http_proxy") {
        return Some(e);
    }

    None
}

fn create_editor() -> String {
    if let Ok(e) = std::env::var("EDITOR") {
        return e;
    }

    if let Ok(e) = std::env::var("VISUAL") {
        return e;
    }

    if cfg!(target_os = "windows") {
        "%SYSTEMROOT%\notepad.exe".to_string()
    } else {
        "vi".to_string()
    }
}

fn create_browser_env() -> String {
    if cfg!(target_os = "windows") {
        "start".into()
    } else if cfg!(target_os = "macos") {
        "open".into()
    } else {
        "xdg-open".into()
    }
}

fn create_cache_dir() -> String {
    if cfg!(target_os = "windows") {
        "%LocalAppData%\\npm-cache".into()
    } else {
        "~/.npm".into()
    }
}

fn create_script_shell() -> String {
    if cfg!(target_os = "windows") {
        "cmd.exe".into()
    } else {
        "/bin/sh".into()
    }
}

fn create_shell() -> String {
    if let Ok(e) = std::env::var("SHELL") {
        return e;
    }

    if cfg!(target_os = "windows") {
        "cmd.exe".to_string()
    } else {
        "bash".to_string()
    }
}

fn create_unicode() -> bool {
    !cfg!(target_os = "windows")
}

fn create_viewer() -> String {
    if cfg!(target_os = "windows") {
        "browser".to_string()
    } else {
        "man".to_string()
    }
}

impl NpmConfig {
    // https://docs.npmjs.com/cli/v10/using-npm/config
    pub fn new(conf: BTreeMap<String, Option<String>>) -> Self {
        let npm_config_defaults = NpmConfig {
            _auth: None,
            access: Access::Public,
            all: false,
            allow_same_version: false,
            audit: true,
            audit_level: None,
            auth_type: AuthType::Web,
            before: None,
            bin_links: true,
            browser: create_browser_env(),
            ca: None,
            cache: create_cache_dir(),
            ca_file: None,
            call: None,
            cidr: None,
            color: true,
            commit_hooks: true,
            cpu: None,
            depth: Depth::IntVal(1),
            description: true,
            diff: "".into(),
            diff_dst_prefix: "b/".into(),
            diff_ignore_all_space: false,
            diff_name_only: false,
            diff_no_prefix: false,
            diff_src_prefix: "a/".into(),
            diff_text: false,
            diff_unified: 3,
            dry_run: false,
            editor: create_editor(),
            engine_strict: false,
            expect_results: None,
            expect_result_count: None,
            fetch_retries: 2,
            fetch_retry_factor: 10,
            fetch_retry_maxtimeout: 60000,
            fetch_retry_mintimeout: 10000,
            fetch_timeout: 300000,
            force: false,
            foreground_scripts: false,
            format_package_lock: true,
            fund: true,
            git: "git".into(),
            git_tag_version: true,
            global: false,
            globalconfig: None,
            heading: "npm".into(),
            https_proxy: create_https_proxy_conf(),
            if_present: false,
            ignore_scripts: false,
            include: None,
            include_staged: false,
            include_workspace_root: false,
            init_author_email: None,
            init_author_name: None,
            init_author_url: None,
            init_license: "ISC".into(),
            init_module: "~/.npm-init.js".into(),
            init_version: "1.0.0".into(),
            install_links: false,
            install_strategy: InstallStrategy::Hoisted,
            json: false,
            legacy_peer_deps: false,
            libc: None,
            link: false,
            local_address: None,
            lockfile_version: 9,
            _location: Location::User,
            _log_level: LogLevel::Notice,
            logs_dir: "_logs".into(),
            logs_max: 10,
            long: false,
            max_sockets: 15,
            message: "%s".into(),
            node_options: create_node_options(),
            no_proxy: create_no_proxy(),
            offline: false,
            omit: create_omit(),
            omit_lockfile_registry_resolved: false,
            os: None,
            otp: None,
            pack_destination: Some(".".into()),
            package: "".to_string(),
            package_lock: true,
            package_lock_only: false,
            parseable: false,
            prefer_dedupe: false,
            prefer_offline: false,
            prefer_online: false,
            prefix: "".into(),
            preid: None,
            progress: true,
            provenance: false,
            provenance_file: None,
            proxy: None,
            read_only: false,
            rebuild_bundle: true,
            registry: "https://registry.npmjs.org/".to_string(),
            replace_registry_host: "npmjs".to_string(),
            save: true,
            save_bundle: false,
            save_dev: false,
            save_exact: false,
            save_optional: false,
            save_peer: false,
            save_prefix: "^".to_string(),
            save_prod: false,
            sbom_format: None,
            sbom_type: "library".to_string(),
            scope: None,
            script_shell: create_script_shell(),
            search_exclude: None,
            search_limit: 20,
            search_opts: None,
            search_staleness: 900,
            shell: create_shell(),
            sign_git_commit: false,
            sign_git_tag: false,
            strict_peer_deps: false,
            strict_ssl: true,
            tag: "latest".to_string(),
            tag_version_prefix: "v".to_string(),
            timing: false,
            umask: 0,
            unicode: create_unicode(),
            update_notifier: true,
            usage: false,
            user_agent: "npm/{npm-version} node/{node-version} {platform} {arch} workspaces/{workspaces} {ci}".to_string(),
            user_config:  "~/.npmrc".to_string(),
            version: false,
            versions: false,
            viewer: create_viewer(),
            which: None,
            workspace: None,
            workspaces: None,
            workspaces_update: true,
            yes: None,
            also: None,
            cache_max: 20000000,
            cache_min: 0,
            cert: None,
            _key: None,
        };

        let mut conf_struct = npm_config_defaults;

        Self::determine_config(&mut conf_struct, conf);

        println!("{:?}", conf_struct);
        conf_struct
    }

    fn parse_bool(default_value: bool, value: &Option<String>) -> bool {
        if value.is_none() {
            return default_value;
        }

        println!("{:?}", value.clone().unwrap().as_str());

        match value.clone().unwrap().as_str() {
            "true" => true,
            "false" => false,
            _ => default_value,
        }
    }

    fn parse_string(default_value: &Option<String>, value: &Option<String>) -> Option<String> {
        if value.is_none() {
            return default_value.clone();
        }
        value.clone()
    }

    fn parse_date(default_value: Option<NaiveDate>, value: &Option<String>) -> Option<NaiveDate> {
        if value.is_none() {
            return default_value;
        }
        match NaiveDate::parse_from_str(value.clone().unwrap().as_str(), "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => default_value,
        }
    }

    fn parse_set_string(default_value: &str, value: &Option<String>) -> String {
        if value.is_none() {
            return default_value.to_string();
        }
        value.clone().unwrap()
    }

    fn determine_config(conf_struct: &mut NpmConfig, conf: BTreeMap<String, Option<String>>) {
        for (key, value) in conf {
            if value.is_none() {
                continue;
            }
            Self::handle_key_processing(conf_struct, key, &value);
        }
    }

    fn handle_key_processing(conf_struct: &mut NpmConfig, key: String, value: &Option<String>) {
        match key.as_str() {
            AUTH => {
                conf_struct._auth = Self::parse_string(&conf_struct._auth, value);
            }
            ACCESS => {
                if let Some(v) = value {
                    match v.as_str() {
                        "public" => conf_struct.access = Access::Public,
                        "restricted" => conf_struct.access = Access::Restricted,
                        _ => conf_struct.access = Access::Null,
                    }
                }
            }
            ALL => {
                conf_struct.all = Self::parse_bool(conf_struct.all, value);
            }
            ALLOW_SAME_VERSION => {
                conf_struct.allow_same_version =
                    Self::parse_bool(conf_struct.allow_same_version, value);
            }
            AUDIT => {
                conf_struct.audit = Self::parse_bool(conf_struct.audit, value);
            }
            AUDIT_LEVEL => {
                conf_struct.audit_level = Self::parse_string(&conf_struct.audit_level, value);
            }
            AUTH_TYPE => match value.clone().unwrap().as_str() {
                "web" => conf_struct.auth_type = AuthType::Web,
                "legacy" => conf_struct.auth_type = AuthType::Legacy,
                _ => conf_struct.auth_type = AuthType::Web,
            },
            BEFORE => {
                conf_struct.before = Self::parse_date(conf_struct.before, value);
            }
            BIN_LINKS => {
                conf_struct.bin_links = Self::parse_bool(conf_struct.bin_links, value);
            }
            BROWSER => {
                conf_struct.browser = Self::parse_set_string(&conf_struct.browser, value);
            }
            CA => {
                conf_struct.ca = Self::parse_string(&conf_struct.ca, value);
            }
            CACHE => {
                conf_struct.cache = Self::parse_set_string(&conf_struct.cache, value);
            }
            CA_FILE => {
                conf_struct.ca_file = Self::parse_string(&conf_struct.ca_file, value);
            }
            CALL => {
                conf_struct.call = Self::parse_string(&conf_struct.call, value);
            }
            CIDR => {
                conf_struct.cidr = Self::parse_string(&conf_struct.cidr, value);
            }
            COLOR => {
                conf_struct.color = Self::parse_bool(conf_struct.color, value);
            }
            COMMIT_HOOKS => {
                conf_struct.commit_hooks = Self::parse_bool(conf_struct.commit_hooks, value);
            }
            CPU => {
                conf_struct.cpu = Self::parse_string(&conf_struct.cpu, value);
            }
            DEPTH => {
                if let Some(val) = value {
                    if val.parse::<i32>().is_ok() {
                        conf_struct.depth = Depth::IntVal(val.parse().unwrap());
                    } else {
                        conf_struct.depth = Depth::StringVal(val.clone());
                    }
                }
            }
            DESCRIPTION => {
                conf_struct.description = Self::parse_bool(conf_struct.description, value);
            }
            DIFF => {
                conf_struct.diff = Self::parse_set_string(&conf_struct.diff, value);
            }
            DIFF_DST_PREFIX => {
                conf_struct.diff_dst_prefix =
                    Self::parse_set_string(&conf_struct.diff_dst_prefix, value);
            }
            DIFF_IGNORE_ALL_SPACE => {
                conf_struct.diff_ignore_all_space =
                    Self::parse_bool(conf_struct.diff_ignore_all_space, value);
            }
            DIFF_NAME_ONLY => {
                conf_struct.diff_name_only = Self::parse_bool(conf_struct.diff_name_only, value);
            }
            DIFF_NO_PREFIX => {
                conf_struct.diff_no_prefix = Self::parse_bool(conf_struct.diff_no_prefix, value);
            }
            DIFF_SRC_PREFIX => {
                conf_struct.diff_src_prefix =
                    Self::parse_set_string(&conf_struct.diff_src_prefix, value);
            }
            DIFF_TEXT => {
                conf_struct.diff_text = Self::parse_bool(conf_struct.diff_text, value);
            }
            DIFF_UNIFIED => {
                if let Ok(val) = value.clone().unwrap().parse::<i32>() {
                    conf_struct.diff_unified = val;
                }
            }
            DRY_RUN => {
                conf_struct.dry_run = Self::parse_bool(conf_struct.dry_run, value);
            }
            EDITOR => {
                conf_struct.editor = Self::parse_set_string(&conf_struct.editor, value);
            }
            ENGINE_STRICT => {
                conf_struct.engine_strict = Self::parse_bool(conf_struct.engine_strict, value);
            }
            EXPECT_RESULT_COUNT => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.expect_result_count = Some(val);
                    }
                }
            }
            EXPECT_RESULTS => {
                conf_struct.expect_results = Some(Self::parse_bool(false, value));
            }
            FETCH_RETRIES => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.fetch_retries = val;
                    }
                }
            }
            FETCH_RETRY_FACTOR => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.fetch_retry_factor = val;
                    }
                }
            }
            FETCH_RETRY_MAXTIMEOUT => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.fetch_retry_maxtimeout = val;
                    }
                }
            }
            FETCH_RETRY_MINTIMEOUT => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.fetch_retry_mintimeout = val;
                    }
                }
            }
            FETCH_TIMEOUT => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.fetch_timeout = val;
                    }
                }
            }
            FORCE => {
                conf_struct.force = Self::parse_bool(conf_struct.force, value);
            }
            FOREGROUND_SCRIPTS => {
                conf_struct.foreground_scripts =
                    Self::parse_bool(conf_struct.foreground_scripts, value);
            }
            FORMAT_PACKAGE_LOCK => {
                conf_struct.format_package_lock =
                    Self::parse_bool(conf_struct.format_package_lock, value);
            }
            FUND => {
                conf_struct.fund = Self::parse_bool(conf_struct.fund, value);
            }
            GIT => {
                conf_struct.git = Self::parse_set_string(&conf_struct.git, value);
            }
            GIT_TAG_VERSION => {
                conf_struct.git_tag_version = Self::parse_bool(conf_struct.git_tag_version, value);
            }
            GLOBAL => {
                conf_struct.global = Self::parse_bool(conf_struct.global, value);
            }
            GLOBAL_CONFIG => {
                conf_struct.globalconfig = Self::parse_string(&conf_struct.globalconfig, value);
            }
            HEADING => {
                conf_struct.heading = Self::parse_set_string(&conf_struct.heading, value);
            }
            HTTPS_PROXY => {
                conf_struct.https_proxy = Self::parse_string(&conf_struct.https_proxy, value);
            }
            IF_PRESENT => {
                conf_struct.if_present = Self::parse_bool(conf_struct.if_present, value);
            }
            IGNORE_SCRIPTS => {
                conf_struct.ignore_scripts = Self::parse_bool(conf_struct.ignore_scripts, value);
            }
            INCLUDE => {
                if let Some(v) = value {
                    match v.as_str() {
                        "dev" => conf_struct.include = Some(Include::Dev),
                        "optional" => conf_struct.include = Some(Include::Optional),
                        "prod" => conf_struct.include = Some(Include::Prod),
                        "peer" => conf_struct.include = Some(Include::Peer),
                        _ => conf_struct.include = None,
                    }
                }
            }
            INCLUDE_STAGED => {
                conf_struct.include_staged = Self::parse_bool(conf_struct.include_staged, value);
            }
            INCLUDE_WORKSPACE_ROOT => {
                conf_struct.include_workspace_root =
                    Self::parse_bool(conf_struct.include_workspace_root, value);
            }
            INIT_AUTHOR_EMAIL => {
                conf_struct.init_author_email =
                    Self::parse_string(&conf_struct.init_author_email, value);
            }
            INIT_AUTHOR_NAME => {
                conf_struct.init_author_name =
                    Self::parse_string(&conf_struct.init_author_name, value);
            }
            INIT_AUTHOR_URL => {
                conf_struct.init_author_url =
                    Self::parse_string(&conf_struct.init_author_url, value);
            }
            INIT_LICENSE => {
                conf_struct.init_license = Self::parse_set_string(&conf_struct.init_license, value);
            }
            INIT_MODULE => {
                conf_struct.init_module = Self::parse_set_string(&conf_struct.init_module, value);
            }
            INIT_VERSION => {
                conf_struct.init_version = Self::parse_set_string(&conf_struct.init_version, value);
            }
            INSTALL_LINKS => {
                conf_struct.install_links = Self::parse_bool(conf_struct.install_links, value);
            }
            INSTALL_STRATEGY => {
                if let Some(v) = value {
                    match v.as_str() {
                        "hoisted" => conf_struct.install_strategy = InstallStrategy::Hoisted,
                        "nested" => conf_struct.install_strategy = InstallStrategy::Nested,
                        "shallow" => conf_struct.install_strategy = InstallStrategy::Shallow,
                        "linked" => conf_struct.install_strategy = InstallStrategy::Linked,
                        _ => conf_struct.install_strategy = InstallStrategy::Hoisted,
                    }
                }
            }
            JSON => {
                conf_struct.json = Self::parse_bool(conf_struct.json, value);
            }
            LEGACY_PEER_DEPS => {
                conf_struct.legacy_peer_deps =
                    Self::parse_bool(conf_struct.legacy_peer_deps, value);
            }
            LIBC => {
                conf_struct.libc = Self::parse_string(&conf_struct.libc, value);
            }
            LINK => {
                conf_struct.link = Self::parse_bool(conf_struct.link, value);
            }
            LOCAL_ADDRESS => {
                conf_struct.local_address = Self::parse_string(&conf_struct.local_address, value);
            }
            LOCATION => {
                if let Some(v) = value {
                    match v.as_str() {
                        "user" => conf_struct._location = Location::User,
                        "global" => conf_struct._location = Location::Global,
                        "project" => conf_struct._location = Location::Project,
                        _ => conf_struct._location = Location::User,
                    }
                }
            }
            LOCKFILE_VERSION => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.lockfile_version = val;
                    }
                }
            }
            LOGLEVEL => {
                if let Some(v) = value {
                    match v.as_str() {
                        "silent" => conf_struct._log_level = LogLevel::Silent,
                        "error" => conf_struct._log_level = LogLevel::Error,
                        "warn" => conf_struct._log_level = LogLevel::Warn,
                        "notice" => conf_struct._log_level = LogLevel::Notice,
                        "verbose" => conf_struct._log_level = LogLevel::Verbose,
                        "http" => conf_struct._log_level = LogLevel::Http,
                        "timing" => conf_struct._log_level = LogLevel::Timing,
                        "info" => conf_struct._log_level = LogLevel::Info,
                        "https" => conf_struct._log_level = LogLevel::Https,
                        "silly" => conf_struct._log_level = LogLevel::Silly,
                        "none" => conf_struct._log_level = LogLevel::None,
                        _ => conf_struct._log_level = LogLevel::Notice,
                    }
                }
            }
            LOGS_DIR => {
                conf_struct.logs_dir = Self::parse_set_string(&conf_struct.logs_dir, value);
            }
            LOGS_MAX => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.logs_max = val;
                    }
                }
            }
            LONG => {
                conf_struct.long = Self::parse_bool(conf_struct.long, value);
            }
            MAX_SOCKETS => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.max_sockets = val;
                    }
                }
            }
            MESSAGE => {
                conf_struct.message = Self::parse_set_string(&conf_struct.message, value);
            }
            NODE_OPTIONS => {
                conf_struct.node_options = Self::parse_string(&conf_struct.node_options, value);
            }
            NO_PROXY => {
                conf_struct.no_proxy = Self::parse_string(&conf_struct.no_proxy, value);
            }
            OFFLINE => {
                conf_struct.offline = Self::parse_bool(conf_struct.offline, value);
            }
            OMIT => {
                conf_struct.omit = Self::parse_string(&conf_struct.omit, value);
            }
            OMIT_LOCKFILE_REGISTRY_RESOLVED => {
                conf_struct.omit_lockfile_registry_resolved =
                    Self::parse_bool(conf_struct.omit_lockfile_registry_resolved, value);
            }
            OS => {
                conf_struct.os = Self::parse_string(&conf_struct.os, value);
            }
            OTP => {
                conf_struct.otp = Self::parse_string(&conf_struct.otp, value);
            }
            PACK_DESTINATION => {
                conf_struct.pack_destination =
                    Self::parse_string(&conf_struct.pack_destination, value);
            }
            PACKAGE => {
                conf_struct.package = Self::parse_set_string(&conf_struct.package, value);
            }
            PACKAGE_LOCK => {
                conf_struct.package_lock = Self::parse_bool(conf_struct.package_lock, value);
            }
            PACKAGE_LOCK_ONLY => {
                conf_struct.package_lock_only =
                    Self::parse_bool(conf_struct.package_lock_only, value);
            }
            PARSEABLE => {
                conf_struct.parseable = Self::parse_bool(conf_struct.parseable, value);
            }
            PREFER_DEDUPE => {
                conf_struct.prefer_dedupe = Self::parse_bool(conf_struct.prefer_dedupe, value);
            }
            PREFER_OFFLINE => {
                conf_struct.prefer_offline = Self::parse_bool(conf_struct.prefer_offline, value);
            }
            PREFER_ONLINE => {
                conf_struct.prefer_online = Self::parse_bool(conf_struct.prefer_online, value);
            }
            PREFIX => {
                conf_struct.prefix = Self::parse_set_string(&conf_struct.prefix, value);
            }
            PREID => {
                conf_struct.preid = Self::parse_string(&conf_struct.preid, value);
            }
            PROGRESS => {
                conf_struct.progress = Self::parse_bool(conf_struct.progress, value);
            }
            PROVENANCE => {
                conf_struct.provenance = Self::parse_bool(conf_struct.provenance, value);
            }
            PROVENANCE_FILE => {
                conf_struct.provenance_file =
                    Self::parse_string(&conf_struct.provenance_file, value);
            }
            PROXY => {
                conf_struct.proxy = Self::parse_string(&conf_struct.proxy, value);
            }
            READ_ONLY => {
                conf_struct.read_only = Self::parse_bool(conf_struct.read_only, value);
            }
            REBUILD_BUNDLE => {
                conf_struct.rebuild_bundle = Self::parse_bool(conf_struct.rebuild_bundle, value);
            }
            REGISTRY => {
                conf_struct.registry = Self::parse_set_string(&conf_struct.registry, value);
            }
            REPLACE_REGISTRY_HOST => {
                conf_struct.replace_registry_host =
                    Self::parse_set_string(&conf_struct.replace_registry_host, value);
            }
            SAVE => {
                conf_struct.save = Self::parse_bool(conf_struct.save, value);
            }
            SAVE_BUNDLE => {
                conf_struct.save_bundle = Self::parse_bool(conf_struct.save_bundle, value);
            }
            SAVE_DEV => {
                conf_struct.save_dev = Self::parse_bool(conf_struct.save_dev, value);
            }
            SAVE_EXACT => {
                conf_struct.save_exact = Self::parse_bool(conf_struct.save_exact, value);
            }
            SAVE_OPTIONAL => {
                conf_struct.save_optional = Self::parse_bool(conf_struct.save_optional, value);
            }
            SAVE_PEER => {
                conf_struct.save_peer = Self::parse_bool(conf_struct.save_peer, value);
            }
            SAVE_PREFIX => {
                conf_struct.save_prefix = Self::parse_set_string(&conf_struct.save_prefix, value);
            }
            SAVE_PROD => {
                conf_struct.save_prod = Self::parse_bool(conf_struct.save_prod, value);
            }
            SBOM_FORMAT => {
                conf_struct.sbom_format = Self::parse_string(&conf_struct.sbom_format, value);
            }
            SBOM_TYPE => {
                conf_struct.sbom_type = Self::parse_set_string(&conf_struct.sbom_type, value);
            }
            SCOPE => {
                conf_struct.scope = Self::parse_string(&conf_struct.scope, value);
            }
            SCRIPT_SHELL => {
                conf_struct.script_shell = Self::parse_set_string(&conf_struct.script_shell, value);
            }
            SEARCH_EXCLUDE => {
                conf_struct.search_exclude = Self::parse_string(&conf_struct.search_exclude, value);
            }
            SEARCH_LIMIT => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.search_limit = val;
                    }
                }
            }
            SEARCH_OPTS => {
                conf_struct.search_opts = Self::parse_string(&conf_struct.search_opts, value);
            }
            SEARCH_STALENESS => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.search_staleness = val;
                    }
                }
            }
            SHELL => {
                conf_struct.shell = Self::parse_set_string(&conf_struct.shell, value);
            }
            SIGN_GIT_COMMIT => {
                conf_struct.sign_git_commit = Self::parse_bool(conf_struct.sign_git_commit, value);
            }
            SIGN_GIT_TAG => {
                conf_struct.sign_git_tag = Self::parse_bool(conf_struct.sign_git_tag, value);
            }
            STRICT_PEER_DEPS => {
                conf_struct.strict_peer_deps =
                    Self::parse_bool(conf_struct.strict_peer_deps, value);
            }
            STRICT_SSL => {
                conf_struct.strict_ssl = Self::parse_bool(conf_struct.strict_ssl, value);
            }
            TAG => {
                conf_struct.tag = Self::parse_set_string(&conf_struct.tag, value);
            }
            TAG_VERSION_PREFIX => {
                conf_struct.tag_version_prefix =
                    Self::parse_set_string(&conf_struct.tag_version_prefix, value);
            }
            TIMING => {
                conf_struct.timing = Self::parse_bool(conf_struct.timing, value);
            }
            UMASK => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.umask = val;
                    }
                }
            }
            UNICODE => {
                conf_struct.unicode = Self::parse_bool(conf_struct.unicode, value);
            }
            UPDATE_NOTIFIER => {
                conf_struct.update_notifier = Self::parse_bool(conf_struct.update_notifier, value);
            }
            USAGE => {
                conf_struct.usage = Self::parse_bool(conf_struct.usage, value);
            }
            USER_AGENT => {
                conf_struct.user_agent = Self::parse_set_string(&conf_struct.user_agent, value);
            }
            USER_CONFIG => {
                conf_struct.user_config = Self::parse_set_string(&conf_struct.user_config, value);
            }
            VERSION => {
                conf_struct.version = Self::parse_bool(conf_struct.version, value);
            }
            VERSIONS => {
                conf_struct.versions = Self::parse_bool(conf_struct.versions, value);
            }
            VIEWER => {
                conf_struct.viewer = Self::parse_set_string(&conf_struct.viewer, value);
            }
            WHICH => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.which = Some(val);
                    }
                }
            }
            WORKSPACE => {
                conf_struct.workspace = Self::parse_string(&conf_struct.workspace, value);
            }
            WORKSPACES => {
                conf_struct.workspaces = Some(Self::parse_bool(false, value));
            }
            WORKSPACES_UPDATE => {
                conf_struct.workspaces_update =
                    Self::parse_bool(conf_struct.workspaces_update, value);
            }
            YES => {
                conf_struct.yes = Some(Self::parse_bool(false, value));
            }
            ALSO => {
                conf_struct.also = Self::parse_string(&conf_struct.also, value);
            }
            CACHE_MAX => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.cache_max = val;
                    }
                }
            }
            CACHE_MIN => {
                if let Some(val) = value {
                    if let Ok(val) = val.parse::<i32>() {
                        conf_struct.cache_min = val;
                    }
                }
            }
            CERT => {
                conf_struct.cert = Self::parse_string(&conf_struct.cert, value);
            }
            _ => {
                log::debug!("Unknown key: {}", key);
            }
        }
    }

    pub fn switch_global(&mut self, global_val: Option<String>) {
        if global_val.is_none() {
            return;
        }
        self.global = Self::parse_bool(false, &global_val);
    }

    pub fn switch_location(&mut self, location_val: Option<String>) {
        if location_val.is_none() {
            return;
        }
        match location_val.unwrap().as_str() {
            "user" => self._location = Location::User,
            "global" => self._location = Location::Global,
            "project" => self._location = Location::Project,
            _ => self._location = Location::User,
        }
    }

    fn serialize_config(map: BTreeMap<String, Option<String>>) -> String {
        let mut serialized = String::new();
        for (key, value) in map {
            if key.is_empty() {
                continue;
            }
            match value {
                Some(v) => {
                    serialized.push_str(&format!("{}={}\n", key, v));
                }
                None => {
                    serialized.push_str(&format!("{}=\n", key));
                }
            }
        }
        serialized
    }

    pub fn set_value(&mut self, key: &str, value: Option<String>) -> Result<(), ExecutionError> {
        Self::handle_key_processing(self, key.to_string(), &value);
        if self._location == Location::Global {
            self.global = true;
        }

        if self._location == Location::User {
            let conf_file = determine_global_config_file_location();
            let conf_content = std::fs::read_to_string(&conf_file).unwrap();
            let mut read_conf = parse_config(conf_content);

            match value {
                Some(v) => {
                    read_conf.insert(key.to_string(), Some(v));
                }
                None => {
                    read_conf.insert(key.to_string(), None);
                }
            }
            let serialized_config = Self::serialize_config(read_conf);
            std::fs::write(conf_file, serialized_config).unwrap();
        }
        Ok(())
    }

    pub fn get_value(&mut self, key: String) -> Result<(), ExecutionError> {
        if self._location == Location::User {
            let read_conf_string =
                std::fs::read_to_string(determine_global_config_file_location()).unwrap();
            let conf = parse_config(read_conf_string);
            let retrieved_kv = conf.get(&key);
            match retrieved_kv {
                Some(v) => {
                    println!("{}", v.clone().unwrap());
                }
                None => {
                    println!("undefined");
                }
            }
        }
        Ok(())
    }

    pub fn list_value(&self) -> Result<(), ExecutionError> {
        if self._location == Location::User {
            let read_conf_string =
                std::fs::read_to_string(determine_global_config_file_location()).unwrap();
            let conf = parse_config(read_conf_string);

            if self.json {
                let serialized_json = serde_json::to_string_pretty(&conf).unwrap();
                println!("{}", serialized_json);
                return Ok(());
            }

            for (key, value) in conf {
                if key.is_empty() {
                    continue;
                }
                match value {
                    Some(v) => {
                        println!("{}={}", key, v);
                    }
                    None => {
                        println!("{}=", key);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn switch_json(&mut self, json_val: Option<bool>) {
        if json_val.is_none() {
            return;
        }
        self.json = json_val.unwrap();
    }
}
