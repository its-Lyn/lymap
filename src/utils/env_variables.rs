pub fn get_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to fetch..");
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
