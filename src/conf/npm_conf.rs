use std::collections::HashMap;
use std::string::ToString;
use chrono::NaiveDate;
use crate::conf::constants;

pub enum Access {
    Public,
    Restricted,
    Null,
}

pub enum AuthType {
    Web,
    Legacy,
}

pub enum Depth {
    StringVal(String),
    IntVal(i32),
}


pub enum Include {
    Dev,
    Optional,
    Prod,
    Peer,
}

pub struct NpmConfig {
    _auth: Option<String>,
    access: Access,
    all: bool,
    allow_same_version: bool,
    audit: bool,
    audit_level: Option<String>,
    auth_type: AuthType,
    before: Option<NaiveDate>,
    bin_links: bool,
    browser: String,
    ca: Option<String>,
    cache: String,
    ca_file: Option<String>,
    call: Option<String>,
    cidr: Option<String>,
    color: bool,
    commit_hooks: bool,
    cpu: Option<String>,
    depth: Depth,
    description: bool,
    diff: String,
    diff_dst_prefix: String,
    diff_ignore_all_space: bool,
    diff_name_only: bool,
    diff_no_prefix: bool,
    diff_src_prefix: String,
    diff_text: bool,
    diff_unified: i32,
    dry_run: bool,
    editor: String,
    engine_strict: bool,
    expect_result_count: Option<i32>,
    expect_results: Option<bool>,
    fetch_retries: i32,
    fetch_retry_factor: i32,
    fetch_retry_maxtimeout: i32,
    fetch_retry_mintimeout: i32,
    fetch_timeout: i32,
    force: bool,
    foreground_scripts: bool,
    format_package_lock: bool,
    fund: bool,
    git: String,
    git_tag_version: bool,
    global: bool,
    globalconfig: Option<String>,
    heading: String,
    https_proxy: Option<String>,
    if_present: bool,
    ignore_scripts: bool,
    include: Option<Include>,
    include_staged: bool,
    include_workspace_root: bool,
    init_author_email: Option<String>,
    init_author_name: Option<String>,
    init_author_url: Option<String>,
    init_license: String,
    init_module: String,
    init_version: String,
    install_links: bool,
    install_strategy: InstallStrategy,
    json: bool,
    legacy_peer_deps: bool,
    libc: Option<String>,
    link: bool,
    local_address: Option<String>,
    location: Location,
    lockfile_version: i32,
    log_level: LogLevel,
    logs_dir: String,
    logs_max: i32,
    long: bool,
    max_sockets: i32,
    message: String,
    node_options: Option<String>,
    no_proxy: Option<String>,
    offline: bool,
    omit: Option<String>,
    omit_lockfile_registry_resolved: bool,
    os: Option<String>,
    otp: Option<String>,
    pack_destination: Option<String>,
    package: String,
    package_lock: bool,
    package_lock_only: bool,
    parseable: bool,
    prefer_dedupe: bool,
    prefer_offline: bool,
    prefer_online: bool,
    prefix: String,
    preid: Option<String>,
    progress: bool,
    provenance: bool,
    provenance_file: Option<String>,
    proxy: Option<String>,
    read_only: bool,
    rebuild_bundle: bool,
    registry: String,
    replace_registry_host: String,
    save: bool,
    save_bundle: bool,
    save_dev: bool,
    save_exact: bool,
    save_optional: bool,
    save_peer: bool,
    save_prefix: String,
    save_prod: bool,
    sbom_format: Option<String>,
    sbom_type: String,
    scope: Option<String>,
    script_shell: String,
    search_exclude: Option<String>,
    search_limit: i32,
    search_opts: Option<String>,
    search_staleness: i32,
    shell: String,
    sign_git_commit: bool,
    sign_git_tag: bool,
    strict_peer_deps: bool,
    strict_ssl: bool,
    tag: String,
    tag_version_prefix: String,
    timing: bool,
    umask: i32,
    unicode: bool,
    update_notifier: bool,
    usage: bool,
    user_agent: String,
    user_config: String,
    version: bool,
    versions: bool,
    viewer: String,
    which: Option<i32>,
    workspace: Option<String>,
    workspaces: Option<bool>,
    workspaces_update: bool,
    yes: Option<bool>,
    also: Option<String>,
    cache_max: i32,
    cache_min: i32,
    cert: Option<String>,
    dev: bool,
    global_style: bool,
    key: Option<String>,
    legacy_bundling: bool,
    only: Option<bool>,
    shrinkwrap: bool
}

enum Location {
    USER,
    GLOBAL,
    PROJECT
}


enum InstallStrategy {
    Hoisted,
    Nested,
    Shallow,
    Linked
}


enum LogLevel {
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
    None
}




fn create_omit() -> Option<String> {
    if let Some(e) = std::env::var("NODE_ENV").ok() {
        if e == "production" {
            return Some("dev".parse().unwrap());
        }
    }

    None
}

fn create_no_proxy() -> Option<String> {
    if let Some(e) = std::env::var("NO_PROXY").ok() {
        return Some(e);
    }

    if let Some(e) = std::env::var("no_proxy").ok() {
        return Some(e);
    }

    None
}


fn create_node_options() -> Option<String> {
    if let Some(e) = std::env::var("NODE_OPTIONS").ok() {
        return Some(e);
    }

    None
}

fn create_https_proxy_conf() -> Option<String> {
    if let Some(e) = std::env::var("HTTPS_PROXY").ok() {
        return Some(e);
    }

    if let Some(e) = std::env::var("https_proxy").ok() {
        return Some(e);
    }

    if let Some(e) = std::env::var("HTTP_PROXY").ok() {
        return Some(e);
    }

    if let Some(e) = std::env::var("http_proxy").ok() {
        return Some(e);
    }

    None
}


fn create_editor() -> String {
    if let Some(e) = std::env::var("EDITOR").ok() {
        return e;
    }

    if let Some(e) = std::env::var("VISUAL").ok() {
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
    if let Some(e) = std::env::var("SHELL").ok() {
        return e;
    }

    if cfg!(target_os = "windows") {
        "cmd.exe".to_string()
    } else {
        "bash".to_string()
    }
}

fn create_unicode() -> bool {
    if cfg!(target_os = "windows") {
        false
    } else {
        true
    }
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
    pub fn new(conf: HashMap<String, Option<String>>) -> Self {

        let npm_config_defaults = NpmConfig {
            _auth: None,
            access: Access::Null,
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
            location: Location::USER,
            log_level: LogLevel::Notice,
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
            dev: false,
            global_style: false,
            key: None,
            legacy_bundling: false,
            only: None,
            shrinkwrap: false,
        };


        let mut conf_struct = npm_config_defaults;

        Self::determine_config(&mut conf_struct, conf);

        conf_struct
    }

    fn parse_bool(default_value: bool, value: Option<String>) -> bool {
        if value.is_none() {
            return default_value;
        }
        match value.unwrap().as_str() {
            "true" => true,
            "false" => false,
            _ => default_value
        }
    }

    fn parse_string(default_value: &Option<String>, value: Option<String>) -> Option<String> {
        if value.is_none() {
            return default_value.clone();
        }
        value
    }

    fn parse_date(default_value: Option<NaiveDate>, value: Option<String>) -> Option<NaiveDate> {
        if value.is_none() {
            return default_value;
        }
        match NaiveDate::parse_from_str(value.unwrap().as_str(), "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => default_value
        }
    }

    fn parse_set_string(default_value: &str, value: Option<String>) -> String {
        if value.is_none() {
            return default_value.to_string();
        }
        value.unwrap()
    }

    fn determine_config(conf_struct: &mut NpmConfig, conf: HashMap<String, Option<String>>) {
        for (key, value) in conf {
            if value.is_none() {
                continue;
            }
            match key.as_str() {
                constants::AUTH=>{
                    conf_struct._auth = Self::parse_string(&conf_struct._auth, value);
                }
                constants::ACCESS=>{
                    conf_struct._auth = Self::parse_string(&conf_struct._auth, value);
                }
                constants::ALL=>{
                    conf_struct.all = Self::parse_bool(conf_struct.all, value);
                }
                constants::ALLOW_SAME_VERSION=>{
                    conf_struct.allow_same_version = Self::parse_bool(conf_struct.allow_same_version, value);
                }
                constants::AUDIT=>{
                    conf_struct.audit = Self::parse_bool(conf_struct.audit, value);
                }
                constants::AUDIT_LEVEL=>{
                    conf_struct.audit_level = Self::parse_string(&conf_struct.audit_level, value);
                }
                constants::AUTH_TYPE=>{
                    match value.unwrap().as_str() {
                        "web" => conf_struct.auth_type = AuthType::Web,
                        "legacy" => conf_struct.auth_type = AuthType::Legacy,
                        _ => conf_struct.auth_type = AuthType::Web
                    }
                }
                constants::BEFORE=>{
                    conf_struct.before = Self::parse_date(conf_struct.before, value);
                }
                constants::BIN_LINKS=>{
                    conf_struct.bin_links = Self::parse_bool(conf_struct.bin_links, value);
                }
                constants::BROWSER=>{
                    conf_struct.browser = Self::parse_set_string(&conf_struct.browser, value);
                }
                constants::CA=>{
                    conf_struct.ca = Self::parse_string(&conf_struct.ca, value);
                }
                constants::CACHE=>{
                    conf_struct.cache = Self::parse_set_string(&conf_struct.cache, value);
                }
                constants::CA_FILE=>{
                    conf_struct.ca_file = Self::parse_string(&conf_struct.ca_file, value);
                }
                constants::CALL=>{
                    conf_struct.call = Self::parse_string(&conf_struct.call, value);
                }
                constants::CIDR=>{
                    conf_struct.cidr = Self::parse_string(&conf_struct.cidr, value);
                }
                constants::COLOR=>{
                    conf_struct.color = Self::parse_bool(conf_struct.color, value);
                }
                constants::COMMIT_HOOKS=>{
                    conf_struct.commit_hooks = Self::parse_bool(conf_struct.commit_hooks, value);
                }
                constants::CPU=>{
                    conf_struct.cpu = Self::parse_string(&conf_struct.cpu, value);
                }
                constants::DEPTH=>{
                    if let Some(val) = value {
                        if val.parse::<i32>().is_ok() {
                            conf_struct.depth = Depth::IntVal(val.parse().unwrap());
                        } else {
                            conf_struct.depth = Depth::StringVal(val);
                        }
                    }
                }
                constants::DESCRIPTION=>{
                    conf_struct.description = Self::parse_bool(conf_struct.description, value);
                }
                constants::DIFF=>{
                    conf_struct.diff = Self::parse_set_string(&conf_struct.diff, value);
                }
                constants::DIFF_DST_PREFIX=>{
                    conf_struct.diff_dst_prefix = Self::parse_set_string(&conf_struct
                        .diff_dst_prefix, value);
                }
                constants::DIFF_IGNORE_ALL_SPACE=>{
                    conf_struct.diff_ignore_all_space = Self::parse_bool(conf_struct.diff_ignore_all_space, value);
                }
                constants::DIFF_NAME_ONLY=>{
                    conf_struct.diff_name_only = Self::parse_bool(conf_struct.diff_name_only, value);
                }
                constants::DIFF_NO_PREFIX=>{
                    conf_struct.diff_no_prefix = Self::parse_bool(conf_struct.diff_no_prefix, value);
                }
                constants::DIFF_SRC_PREFIX=>{
                    conf_struct.diff_src_prefix = Self::parse_set_string(&conf_struct
                        .diff_src_prefix, value);
                }
                constants::DIFF_TEXT=>{
                    conf_struct.diff_text = Self::parse_bool(conf_struct.diff_text, value);
                }
                constants::DIFF_UNIFIED=>{
                    if let Ok(val) = value.unwrap().parse::<i32>() {
                        conf_struct.diff_unified = val;
                    }
                }
                constants::DRY_RUN=>{
                    conf_struct.dry_run = Self::parse_bool(conf_struct.dry_run, value);
                }
                constants::EDITOR=>{
                    conf_struct.editor = Self::parse_set_string(&conf_struct.editor, value);
                }
                constants::ENGINE_STRICT=>{
                    conf_struct.engine_strict = Self::parse_bool(conf_struct.engine_strict, value);
                }
                constants::EXPECT_RESULT_COUNT=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.expect_result_count = Some(val);
                        }
                    }
                }
                constants::EXPECT_RESULTS=>{
                    conf_struct.expect_results = Some(Self::parse_bool(false, value));
                }
                constants::FETCH_RETRIES=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.fetch_retries = val;
                        }
                    }
                },
                constants::FETCH_RETRY_FACTOR=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.fetch_retry_factor = val;
                        }
                    }
                },
                constants::FETCH_RETRY_MAXTIMEOUT=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.fetch_retry_maxtimeout = val;
                        }
                    }
                },
                constants::FETCH_RETRY_MINTIMEOUT=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.fetch_retry_mintimeout = val;
                        }
                    }
                },
                constants::FETCH_TIMEOUT=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.fetch_timeout = val;
                        }
                    }
                },
                constants::FORCE=>{
                    conf_struct.force = Self::parse_bool(conf_struct.force, value);
                }
                constants::FOREGROUND_SCRIPTS=>{
                    conf_struct.foreground_scripts = Self::parse_bool(conf_struct.foreground_scripts, value);
                }
                constants::FORMAT_PACKAGE_LOCK=>{
                    conf_struct.format_package_lock = Self::parse_bool(conf_struct.format_package_lock, value);
                }
                constants::FUND=>{
                    conf_struct.fund = Self::parse_bool(conf_struct.fund, value);
                }
                constants::GIT=>{
                    conf_struct.git = Self::parse_set_string(&conf_struct.git, value);
                }
                constants::GIT_TAG_VERSION=>{
                    conf_struct.git_tag_version = Self::parse_bool(conf_struct.git_tag_version, value);
                }
                constants::GLOBAL=>{
                    conf_struct.global = Self::parse_bool(conf_struct.global, value);
                }
                constants::GLOBAL_CONFIG=>{
                    conf_struct.globalconfig = Self::parse_string(&conf_struct.globalconfig, value);
                }
                constants::HEADING=>{
                    conf_struct.heading = Self::parse_set_string(&conf_struct.heading, value);
                }
                constants::HTTPS_PROXY=>{
                    conf_struct.https_proxy = Self::parse_string(&conf_struct.https_proxy, value);
                }
                constants::IF_PRESENT=>{
                    conf_struct.if_present = Self::parse_bool(conf_struct.if_present, value);
                }
                constants::IGNORE_SCRIPTS=>{
                    conf_struct.ignore_scripts = Self::parse_bool(conf_struct.ignore_scripts, value);
                }
                constants::INCLUDE=>{
                    if let Some(v) = value {
                        match v.as_str() {
                            "dev" => conf_struct.include = Some(Include::Dev),
                            "optional" => conf_struct.include = Some(Include::Optional),
                            "prod" => conf_struct.include = Some(Include::Prod),
                            "peer" => conf_struct.include = Some(Include::Peer),
                            _ => conf_struct.include = None
                        }
                    }
                },
                constants::INCLUDE_STAGED=>{
                    conf_struct.include_staged = Self::parse_bool(conf_struct.include_staged, value);
                }
                constants::INCLUDE_WORKSPACE_ROOT=>{
                    conf_struct.include_workspace_root = Self::parse_bool(conf_struct.include_workspace_root, value);
                }
                constants::INIT_AUTHOR_EMAIL=>{
                    conf_struct.init_author_email = Self::parse_string(&conf_struct.init_author_email, value);
                }
                constants::INIT_AUTHOR_NAME=>{
                    conf_struct.init_author_name = Self::parse_string(&conf_struct.init_author_name, value);
                }
                constants::INIT_AUTHOR_URL=>{
                    conf_struct.init_author_url = Self::parse_string(&conf_struct.init_author_url, value);
                }
                constants::INIT_LICENSE=>{
                    conf_struct.init_license = Self::parse_set_string(&conf_struct.init_license, value);
                }
                constants::INIT_MODULE=>{
                    conf_struct.init_module = Self::parse_set_string(&conf_struct.init_module, value);
                }
                constants::INIT_VERSION=>{
                    conf_struct.init_version = Self::parse_set_string(&conf_struct.init_version,
                                                                       value);
                }
                constants::INSTALL_LINKS=>{
                    conf_struct.install_links = Self::parse_bool(conf_struct.install_links, value);
                }
                constants::INSTALL_STRATEGY=>{
                    if let Some(v) = value {
                        match v.as_str() {
                            "hoisted" => conf_struct.install_strategy = InstallStrategy::Hoisted,
                            "nested" => conf_struct.install_strategy = InstallStrategy::Nested,
                            "shallow" => conf_struct.install_strategy = InstallStrategy::Shallow,
                            "linked" => conf_struct.install_strategy = InstallStrategy::Linked,
                            _ => conf_struct.install_strategy = InstallStrategy::Hoisted
                        }
                    }
                }
                constants::JSON=>{
                    conf_struct.json = Self::parse_bool(conf_struct.json, value);
                }
                constants::LEGACY_PEER_DEPS=>{
                    conf_struct.legacy_peer_deps = Self::parse_bool(conf_struct.legacy_peer_deps, value);
                }
                constants::LIBC=>{
                    conf_struct.libc = Self::parse_string(&conf_struct.libc, value);
                }
                constants::LINK=>{
                    conf_struct.link = Self::parse_bool(conf_struct.link, value);
                }
                constants::LOCAL_ADDRESS=>{
                    conf_struct.local_address = Self::parse_string(&conf_struct.local_address,
                                                                   value);
                }
                constants::LOCATION=>{
                    if let Some(v) = value {
                        match v.as_str() {
                            "user" => conf_struct.location = Location::USER,
                            "global" => conf_struct.location = Location::GLOBAL,
                            "project" => conf_struct.location = Location::PROJECT,
                            _ => conf_struct.location = Location::USER
                        }
                    }
                }
                constants::LOCKFILE_VERSION=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.lockfile_version = val;
                        }
                    }
                }
                constants::LOGLEVEL=>{
                    if let Some(v) = value {
                        match v.as_str() {
                            "silent" => conf_struct.log_level = LogLevel::Silent,
                            "error" => conf_struct.log_level = LogLevel::Error,
                            "warn" => conf_struct.log_level = LogLevel::Warn,
                            "notice" => conf_struct.log_level = LogLevel::Notice,
                            "verbose" => conf_struct.log_level = LogLevel::Verbose,
                            "http" => conf_struct.log_level = LogLevel::Http,
                            "timing" => conf_struct.log_level = LogLevel::Timing,
                            "info" => conf_struct.log_level = LogLevel::Info,
                            "https" => conf_struct.log_level = LogLevel::Https,
                            "silly" => conf_struct.log_level = LogLevel::Silly,
                            "none" => conf_struct.log_level = LogLevel::None,
                            _ => conf_struct.log_level = LogLevel::Notice
                        }
                    }
                }
                constants::LOGS_DIR=>{
                    conf_struct.logs_dir = Self::parse_set_string(&conf_struct.logs_dir, value);
                }
                constants::LOGS_MAX=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.logs_max = val;
                        }
                    }
                }
                constants::LONG=>{
                    conf_struct.long = Self::parse_bool(conf_struct.long, value);
                }
                constants::MAX_SOCKETS=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.max_sockets = val;
                        }
                    }
                }
                constants::MESSAGE=>{
                    conf_struct.message = Self::parse_set_string(&conf_struct.message, value);
                }
                constants::NODE_OPTIONS=>{
                    conf_struct.node_options = Self::parse_string(&conf_struct.node_options, value);
                }
                constants::NO_PROXY=>{
                    conf_struct.no_proxy = Self::parse_string(&conf_struct.no_proxy, value);
                }
                constants::OFFLINE=>{
                    conf_struct.offline = Self::parse_bool(conf_struct.offline, value);
                }
                constants::OMIT=>{
                    conf_struct.omit = Self::parse_string(&conf_struct.omit, value);
                }
                constants::OMIT_LOCKFILE_REGISTRY_RESOLVED=>{
                    conf_struct.omit_lockfile_registry_resolved = Self::parse_bool(conf_struct.omit_lockfile_registry_resolved, value);
                }
                constants::OS=>{
                    conf_struct.os = Self::parse_string(&conf_struct.os, value);
                }
                constants::OTP=>{
                    conf_struct.otp = Self::parse_string(&conf_struct.otp, value);
                }
                constants::PACK_DESTINATION=>{
                    conf_struct.pack_destination = Self::parse_string(&conf_struct.pack_destination, value);
                }
                constants::PACKAGE=>{
                    conf_struct.package = Self::parse_set_string(&conf_struct.package, value);
                }
                constants::PACKAGE_LOCK=>{
                    conf_struct.package_lock = Self::parse_bool(conf_struct.package_lock, value);
                }
                constants::PACKAGE_LOCK_ONLY=>{
                    conf_struct.package_lock_only = Self::parse_bool(conf_struct.package_lock_only, value);
                }
                constants::PARSEABLE=>{
                    conf_struct.parseable = Self::parse_bool(conf_struct.parseable, value);
                }
                constants::PREFER_DEDUPE=>{
                    conf_struct.prefer_dedupe = Self::parse_bool(conf_struct.prefer_dedupe, value);
                }
                constants::PREFER_OFFLINE=>{
                    conf_struct.prefer_offline = Self::parse_bool(conf_struct.prefer_offline, value);
                }
                constants::PREFER_ONLINE=>{
                    conf_struct.prefer_online = Self::parse_bool(conf_struct.prefer_online, value);
                }
                constants::PREFIX=>{
                    conf_struct.prefix = Self::parse_set_string(&conf_struct.prefix, value);
                }
                constants::PREID=>{
                    conf_struct.preid = Self::parse_string(&conf_struct.preid, value);
                }
                constants::PROGRESS=>{
                    conf_struct.progress = Self::parse_bool(conf_struct.progress, value);
                }
                constants::PROVENANCE=>{
                    conf_struct.provenance = Self::parse_bool(conf_struct.provenance, value);
                }
                constants::PROVENANCE_FILE=>{
                    conf_struct.provenance_file = Self::parse_string(&conf_struct.provenance_file, value);
                }
                constants::PROXY=>{
                    conf_struct.proxy = Self::parse_string(&conf_struct.proxy, value);
                }
                constants::READ_ONLY=>{
                    conf_struct.read_only = Self::parse_bool(conf_struct.read_only, value);
                }
                constants::REBUILD_BUNDLE=>{
                    conf_struct.rebuild_bundle = Self::parse_bool(conf_struct.rebuild_bundle, value);
                }
                constants::REGISTRY=>{
                    conf_struct.registry = Self::parse_set_string(&conf_struct.registry, value);
                }
                constants::REPLACE_REGISTRY_HOST=>{
                    conf_struct.replace_registry_host = Self::parse_set_string(&conf_struct.replace_registry_host, value);
                }
                constants::SAVE=>{
                    conf_struct.save = Self::parse_bool(conf_struct.save, value);
                }
                constants::SAVE_BUNDLE=>{
                    conf_struct.save_bundle = Self::parse_bool(conf_struct.save_bundle, value);
                }
                constants::SAVE_DEV=>{
                    conf_struct.save_dev = Self::parse_bool(conf_struct.save_dev, value);
                }
                constants::SAVE_EXACT=>{
                    conf_struct.save_exact = Self::parse_bool(conf_struct.save_exact, value);
                }
                constants::SAVE_OPTIONAL=>{
                    conf_struct.save_optional = Self::parse_bool(conf_struct.save_optional, value);
                }
                constants::SAVE_PEER=>{
                    conf_struct.save_peer = Self::parse_bool(conf_struct.save_peer, value);
                }
                constants::SAVE_PREFIX=>{
                    conf_struct.save_prefix = Self::parse_set_string(&conf_struct.save_prefix, value);
                }
                constants::SAVE_PROD=>{
                    conf_struct.save_prod = Self::parse_bool(conf_struct.save_prod, value);
                }
                constants::SBOM_FORMAT=>{
                    conf_struct.sbom_format = Self::parse_string(&conf_struct.sbom_format, value);
                }
                constants::SBOM_TYPE=>{
                    conf_struct.sbom_type = Self::parse_set_string(&conf_struct.sbom_type, value);
                }
                constants::SCOPE=>{
                    conf_struct.scope = Self::parse_string(&conf_struct.scope, value);
                }
                constants::SCRIPT_SHELL=>{
                    conf_struct.script_shell = Self::parse_set_string(&conf_struct.script_shell, value);
                }
                constants::SEARCH_EXCLUDE=>{
                    conf_struct.search_exclude = Self::parse_string(&conf_struct.search_exclude, value);
                }
                constants::SEARCH_LIMIT=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.search_limit = val;
                        }
                    }
                }
                constants::SEARCH_OPTS=>{
                    conf_struct.search_opts = Self::parse_string(&conf_struct.search_opts, value);
                }
                constants::SEARCH_STALENESS=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.search_staleness = val;
                        }
                    }
                }
                constants::SHELL=>{
                    conf_struct.shell = Self::parse_set_string(&conf_struct.shell, value);
                }
                constants::SIGN_GIT_COMMIT=>{
                    conf_struct.sign_git_commit = Self::parse_bool(conf_struct.sign_git_commit, value);
                }
                constants::SIGN_GIT_TAG=>{
                    conf_struct.sign_git_tag = Self::parse_bool(conf_struct.sign_git_tag, value);
                }
                constants::STRICT_PEER_DEPS=>{
                    conf_struct.strict_peer_deps = Self::parse_bool(conf_struct.strict_peer_deps, value);
                }
                constants::STRICT_SSL=>{
                    conf_struct.strict_ssl = Self::parse_bool(conf_struct.strict_ssl, value);
                }
                constants::TAG=>{
                    conf_struct.tag = Self::parse_set_string(&conf_struct.tag, value);
                }
                constants::TAG_VERSION_PREFIX=>{
                    conf_struct.tag_version_prefix = Self::parse_set_string(&conf_struct.tag_version_prefix, value);
                }
                constants::TIMING=>{
                    conf_struct.timing = Self::parse_bool(conf_struct.timing, value);
                }
                constants::UMASK=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.umask = val;
                        }
                    }
                }
                constants::UNICODE=>{
                    conf_struct.unicode = Self::parse_bool(conf_struct.unicode, value);
                }
                constants::UPDATE_NOTIFIER=>{
                    conf_struct.update_notifier = Self::parse_bool(conf_struct.update_notifier, value);
                }
                constants::USAGE=>{
                    conf_struct.usage = Self::parse_bool(conf_struct.usage, value);
                }
                constants::USER_AGENT=>{
                    conf_struct.user_agent = Self::parse_set_string(&conf_struct.user_agent, value);
                }
                constants::USER_CONFIG=>{
                    conf_struct.user_config = Self::parse_set_string(&conf_struct.user_config, value);
                },
                constants::VERSION=>{
                    conf_struct.version = Self::parse_bool(conf_struct.version, value);
                }
                constants::VERSIONS=>{
                    conf_struct.versions = Self::parse_bool(conf_struct.versions, value);
                }
                constants::VIEWER=>{
                    conf_struct.viewer = Self::parse_set_string(&conf_struct.viewer, value);
                }
                constants::WHICH=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.which = Some(val);
                        }
                    }
                }
                constants::WORKSPACE=>{
                    conf_struct.workspace = Self::parse_string(&conf_struct.workspace, value);
                }
                constants::WORKSPACES=>{
                    conf_struct.workspaces = Some(Self::parse_bool(false, value));
                }
                constants::WORKSPACES_UPDATE=>{
                    conf_struct.workspaces_update = Self::parse_bool(conf_struct.workspaces_update, value);
                }
                constants::YES=>{
                    conf_struct.yes = Some(Self::parse_bool(false, value));
                }
                constants::ALSO=>{
                    conf_struct.also = Self::parse_string(&conf_struct.also, value);
                }
                constants::CACHE_MAX=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.cache_max = val;
                        }
                    }
                }
                constants::CACHE_MIN=>{
                    if let Some(val) = value {
                        if let Ok(val) = val.parse::<i32>() {
                            conf_struct.cache_min = val;
                        }
                    }
                }
                constants::CERT=>{
                    conf_struct.cert = Self::parse_string(&conf_struct.cert, value);
                }
                _=>{
                    println!("Unknown key: {}", key);
                }
            }
        }
    }
}