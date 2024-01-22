use std::error::Error;
use log::warn;

use crate::{utils::config_path::get_config_path, config::models::ButtonConfig};

use super::model::Cache;

pub fn load(button_config: &mut Option<ButtonConfig>) -> Result<(), Box<dyn Error>> {
    fn fetch_config(button_config: &mut Option<ButtonConfig>, cache_source: &String) -> Result<(), Box<dyn Error>> {
        let cache = serde_json::from_str::<Cache>(cache_source)?;
    
        let config_source = std::fs::read_to_string(&cache.last_layout)?;
        let config = serde_json::from_str(&config_source)?;
        *button_config = config;
    
        Ok(())
    }
    
    let cache_path = format!("{}/cache/cache.json", get_config_path()?);
    let cache_source = std::fs::read_to_string(&cache_path)?;

    if let Err(e) = fetch_config(button_config, &cache_source) {
        warn!("Failed to load cache: {}. Ignoring.", e);
    }

    Ok(())
}
