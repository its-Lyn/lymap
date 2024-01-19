use crate::utils::env_variables::{get_variable, xdg_config_set};

pub fn get_config_path() -> String {
    let base_config = if xdg_config_set() { 
        get_variable("XDG_CONFIG_HOME")
    } else { 
        format!("{}/.config", get_variable("HOME"))
    };

    format!("{}/lymap", base_config)
}
