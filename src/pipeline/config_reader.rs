use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use homedir::my_home;

const CONFIG_PNPM: &str = "pnpm/rc";

pub fn determine_config_file_location() -> PathBuf {
    if env::var("$XDG_CONFIG_HOME").is_ok() {
        let mut config_dir = PathBuf::from(env::var("$XDG_CONFIG_HOME").unwrap());
        config_dir.push(CONFIG_PNPM);
        return config_dir;
    }

    let mut home_dir = my_home().unwrap().unwrap();

    if cfg!(target_os = "windows") {
        home_dir.push("AppData");
        home_dir.push("Local");
        home_dir.push("pnpm");
        home_dir.push("config");
        home_dir.push("rc");
        home_dir
    } else if cfg!(target_os = "macos") {
        home_dir.push("Library");
        home_dir.push("Preferences");
        home_dir.push("pnpm");
        home_dir.push("rc");
        home_dir
    } else if cfg!(target_os = "linux") {
        home_dir.push(".config");
        home_dir.push("pnpm");
        home_dir.push("rc");
        home_dir
    } else {
        home_dir.push(".config");
        home_dir.push("pnpm");
        home_dir.push("rc");
        home_dir
    }
}

fn parse_config(conf: String) -> HashMap<String, Option<String>> {
    let mut config_map = HashMap::new();
    let lines = conf.split("\n");
    for line in lines {
        let parts = line.split("=");
        let mut parts_iter = parts.into_iter();

        // skip invalid line
        if parts_iter.clone().count() < 1 {
            continue;
        }

        let key = parts_iter.next().unwrap();
        let value = parts_iter.next();
        config_map.insert(key.to_string(), value.map(|s| s.to_string()));
    }
    config_map
}

pub fn read_config_file() -> Result<HashMap<String, Option<String>>, std::io::Error> {
    let config_file = determine_config_file_location();
    let result_conf_read = std::fs::read_to_string(&config_file);
    match result_conf_read {
        Ok(conf) => {
            Ok(parse_config(conf))
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                std::fs::File::create(&config_file)?;
                return Ok(HashMap::new());
            }
            Err(e)
        }
    }
}