use crate::utils::env_variables::xdg_config_set;

use std::{path::Path, error::Error};
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

pub fn create_config() -> Result<(), Box<dyn Error>> {
    // Set config dir based on user env variables.
    let base_config = if xdg_config_set() {
        warn!("Using XDG_CONFIG_HOME directory");
        std::env::var("XDG_CONFIG_HOME")?
    } else {
        warn!("Using traditional config directory"); 
        format!("{}/.config", std::env::var("HOME")?)
    };

    create_if_not_exists(&base_config);

    // Create lymap folder, AFTER checking if the base exists..
    let lymap_conf = format!("{}/lymap", base_config);
    create_if_not_exists(lymap_conf.as_str());

    // Create window_config.json
    if !Path::new(&format!("{}/window_config.json", lymap_conf)).exists() {
        let default_config = WindowConfig {
            bg_colour: "#0c42a6".to_string(),

            font_path: None,
            font_size: 14,
            font_colour: "#000000".to_string(),
            
            width: None,
            height: None
        };

        info!("{}/window_config.json does not exist, creating.", lymap_conf);

        let file = fs::File::create(format!("{}/window_config.json", lymap_conf))?;
        serde_json::to_writer_pretty(file, &default_config)?;
    }

    // Create key layout folder.
    create_if_not_exists(&format!("{}/layouts", lymap_conf));

    // Crate cache
    create_if_not_exists(&format!("{}/cache", lymap_conf));
   
   if !Path::new(&format!("{}/cache/cache.json", lymap_conf)).exists() {
        info!("{}/cache/cache.json does not exist, creating.", lymap_conf);
        fs::File::create(format!("{}/cache/cache.json", lymap_conf))?;
   } 

    Ok(())
}
