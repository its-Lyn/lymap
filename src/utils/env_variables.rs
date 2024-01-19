use log::error;

pub fn get_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to fetch: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn xdg_config_set() -> bool {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(_) => true,
        Err(_) => false
    }
}
