use homedir::my_home;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

static CRAFT_HOME_DIR: &str = "CRAFT_HOME_DIR";

pub fn get_config_dir(path: PathBuf) -> PathBuf {
    let env_craft_home_dir = env::var(CRAFT_HOME_DIR);
    match env_craft_home_dir {
        Ok(env) => {
            let path_to_folder = PathBuf::from(env).join(path);
            fs::create_dir_all(&path_to_folder).unwrap_or_else(|_| {
                panic!(
                    "Error creating folder {}",
                    path_to_folder.clone().to_str().unwrap()
                )
            });
            path_to_folder
        }
        Err(_) => {
            let home_dir_req = my_home();
            match home_dir_req {
                Ok(e) => match e {
                    Some(home) => {
                        let path_to_folder = home.join(path);
                        fs::create_dir_all(&path_to_folder).unwrap_or_else(|_| {
                            panic!(
                                "Error creating folder {}",
                                path_to_folder.clone().to_str().unwrap()
                            )
                        });
                        path_to_folder
                    }
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
