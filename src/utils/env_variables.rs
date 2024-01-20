pub fn xdg_config_set() -> bool {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(_) => true,
        Err(_) => false
    }
}
