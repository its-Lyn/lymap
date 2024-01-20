use std::env::VarError;

use crate::utils::env_variables::xdg_config_set;

pub fn get_config_path() -> Result<String, VarError> {
    let base_config = if xdg_config_set() { 
        std::env::var("XDG_CONFIG_HOME")?
    } else { 
        format!("{}/.config", std::env::var("HOME")?)
    };

    Ok(format!("{}/lymap", base_config))
}
