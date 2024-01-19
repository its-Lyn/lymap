use crate::utils::env_variables::{xdg_config_set, get_variable};

use std::path::Path;
use std::fs;

use log::{info, error, warn};

use super::models::WindowConfig;

fn create_if_not_exists(path: &str) {
    if !Path::new(path).is_dir() {
        info!("{} does not exist, creating.", path);

        if let Err(e) = fs::create_dir(path) {
            error!("Failed to create config dir: {}, aborting", e);
            std::process::exit(1);
        }
    }
}

pub fn create_config() {
    // Set config dir based on user env variables.
    let base_config = if xdg_config_set() {
        warn!("Using XDG_CONFIG_HOME directory");
        
        get_variable("XDG_CONFIG_HOME")
    } else {
        warn!("Using traditional config directory");
        
        format!("{}/.config", get_variable("HOME"))
    };

    create_if_not_exists(base_config.as_str());

    // Create lymap folder, AFTER checking if the base exists..
    let lymap_conf = format!("{}/lymap", base_config);
    create_if_not_exists(lymap_conf.as_str());

    // Create window_config.json
    if !Path::new(format!("{}/window_config.json", lymap_conf).as_str()).exists() {
        let default_config = WindowConfig {
            bg_colour: "#0c42a6".to_string(),
            
            width: None,
            height: None
        };

        info!("window_config.json does not exist, creating.");

        match fs::File::create(format!("{}/window_config.json", lymap_conf)) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &default_config) {
                    error!("Failed to write to config file: {}, aborting", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                error!("Failed to create config file: {}, aborting", e);
                std::process::exit(1);
            }
        }
    }

    // Create key layout folder.
    create_if_not_exists(
        format!("{}/layouts", lymap_conf).as_str()
    );
}
