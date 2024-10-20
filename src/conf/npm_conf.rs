use std::collections::HashMap;
use std::string::ToString;
use chrono::NaiveDate;

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

pub struct NpmConfig<'a> {
    _auth: Option<String>,
    access: Access,
    all: bool,
    allow_same_version: bool,
    audit: bool,
    audit_level: Option<String>,
    auth_type: AuthType,
    before: Option<NaiveDate>,
    bin_links: bool,
    browser: &'a str,
    ca: Option<String>,
    cache: &'a str,
    ca_file: Option<String>,
    call: Option<String>,
    cidr: Option<String>,
    color: bool,
    commit_hooks: bool,
    cpu: Option<String>,
    depth: Depth,
    description: bool,
    diff: &'a str,
    diff_dst_prefix: &'a str,
    diff_ignore_all_space: bool,
    diff_name_only: bool,
    diff_no_prefix: bool,
    diff_src_prefix: &'a str,
    diff_text: bool,
    diff_unified: i32,
    dry_run: bool,
    editor: &'a str,
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
    git: &'a str,
    git_tag_version: bool,
    global: bool,
    globalconfig: Option<&'a str>,
    heading: &'a str,
    https_proxy: Option<&'a str>,
    if_present: bool,
    ignore_scripts: bool,
    include: Option<Include>,
    include_staged: bool,
    include_workspace_root: bool,
    init_author_email: Option<&'a str>,
    init_author_name: Option<&'a str>,
    init_author_url: Option<&'a str>,
    init_license: &'a str,
    init_module: &'a str,
    init_version: &'a str,
    install_links: bool,
    install_strategy: InstallStrategy,
    json: bool,
    legacy_peer_deps: bool,
    libc: Option<&'a str>,
    link: bool,
    local_address: Option<&'a str>,
    location: Location,
    lockfile_version: i32,
    log_level: LogLevel,
    logs_dir: &'a str,
    logs_max: i32,
    long: bool,
    max_sockets: i32,
    message: &'a str,
    node_options: Option<&'a str>,
    no_proxy: Option<&'a str>,
    offline: bool,
    omit: Option<&'a str>,
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

const NPM_CONFIG_DEFAULTS: NpmConfig = NpmConfig {
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
    diff: "",
    diff_dst_prefix: "b/",
    diff_ignore_all_space: false,
    diff_name_only: false,
    diff_no_prefix: false,
    diff_src_prefix: "a/",
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
    git: "git",
    git_tag_version: true,
    global: false,
    globalconfig: None,
    heading: "npm",
    https_proxy: create_https_proxy_conf(),
    if_present: false,
    ignore_scripts: false,
    include: None,
    include_staged: false,
    include_workspace_root: false,
    init_author_email: None,
    init_author_name: None,
    init_author_url: None,
    init_license: "ISC",
    init_module: "~/.npm-init.js",
    init_version: "1.0.0",
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
    logs_dir: "_logs",
    logs_max: 10,
    long: false,
    max_sockets: 15,
    message: "%s",
    node_options: create_node_options(),
    no_proxy: create_no_proxy(),
    offline: false,
    omit: create_omit(),
};


const fn create_omit() -> Option<&'static str> {
    if let Some(e) = std::env::var("NODE_ENV").ok() {
        if e == "production" {
            return Some("dev");
        }
    }

    None
}

const fn create_no_proxy() -> Option<&'static str> {
    if let Some(e) = std::env::var("NO_PROXY").ok() {
        return Some(e.as_str());
    }

    if let Some(e) = std::env::var("no_proxy").ok() {
        return Some(e.as_str());
    }

    None
}


const fn create_node_options() -> Option<&'static str> {
    if let Some(e) = std::env::var("NODE_OPTIONS").ok() {
        return Some(e.as_str());
    }

    None
}

const fn create_https_proxy_conf() -> Option<&'static str> {
    if let Some(e) = std::env::var("HTTPS_PROXY").ok() {
        return Some(e.as_str());
    }

    if let Some(e) = std::env::var("https_proxy").ok() {
        return Some(e.as_str());
    }

    if let Some(e) = std::env::var("HTTP_PROXY").ok() {
        return Some(e.as_str());
    }

    if let Some(e) = std::env::var("http_proxy").ok() {
        return Some(e.as_str());
    }

    None
}


const fn create_editor() -> &'static str {
    if let Some(e) = std::env::var("EDITOR").ok() {
        return e.as_str();
    }

    if let Some(e) = std::env::var("VISUAL").ok() {
        return e.as_str();
    }


    if cfg!(target_os = "windows") {
        "%SYSTEMROOT%\notepad.exe"
    } else {
        "vi"
    }
}

const fn create_browser_env() -> &'static str {
    if cfg!(target_os = "windows") {
        "start"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
}


const fn create_cache_dir() -> &'static str {
    if cfg!(target_os = "windows") {
        "%LocalAppData%\\npm-cache"
    } else {
        "~/.npm"
    }
}

impl NpmConfig<'_> {
    // https://docs.npmjs.com/cli/v10/using-npm/config
    pub fn new(conf: HashMap<String, Option<String>>) -> Self {
        let mut conf_struct = NPM_CONFIG_DEFAULTS;


        conf_struct
    }
}