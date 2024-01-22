use std::{env::VarError, error::Error, path::Path};

use log::info;

use crate::utils::env_variables::xdg_config_set;

pub fn get_config_path() -> Result<String, VarError> {
    let base_config = if xdg_config_set() { 
        std::env::var("XDG_CONFIG_HOME")?
    } else { 
        format!("{}/.config", std::env::var("HOME")?)
    };

    Ok(format!("{}/lymap", base_config))
}

pub fn get_assets_path() -> Result<String, Box<dyn Error>> {
    let mut asset_path = "/usr/share/lymap/assets".to_string();

    if !Path::new("/usr/share/lymap/assets").exists() {    
        let binary_path = std::env::current_exe()?;
        let binary_dir = binary_path.parent().expect("Failed to get parent of binary.");
        asset_path = format!("{}/assets", binary_dir.display());
    }

    info!("Using asset path: {}", asset_path);        
    Ok(asset_path)
}