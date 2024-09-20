use homedir::my_home;
use std::env;
use std::path::PathBuf;
use std::process::exit;

static CRAFT_HOME_DIR: &str = "CRAFT_HOME_DIR";

pub fn get_config_dir(path: PathBuf) -> PathBuf {
    let env_craft_home_dir = env::var(CRAFT_HOME_DIR);
    match env_craft_home_dir {
        Ok(env) => PathBuf::from(env).join(path),
        Err(_) => {
            let home_dir_req = my_home();
            match home_dir_req {
                Ok(e) => match e {
                    Some(home) => home.join(path),
                    None => {
                        log::error!(
                            "Var {} and home dir not set. Please fix this",
                            CRAFT_HOME_DIR
                        );
                        exit(1)
                    }
                },
                Err(e) => {
                    log::error!("An error occurred while retrieving home dir {}", e);
                    exit(1)
                }
            }
        }
    }
}
