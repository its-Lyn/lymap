use std::error::Error;
use log::{warn, info};

use crate::{utils::config_path::get_config_path, config::models::ButtonConfig};

use super::model::Cache;

pub fn load(button_config: &mut Option<ButtonConfig>) -> Result<(), Box<dyn Error>> {
    let cache_path = format!("{}/cache/cache.json", get_config_path()?);
    let cache_source = std::fs::read_to_string(&cache_path)?;

    match serde_json::from_str::<Cache>(&cache_source) {
        Ok(cache) => {
            let config_source = std::fs::read_to_string(&cache.last_layout)?;
            *button_config = serde_json::from_str::<Option<ButtonConfig>>(&config_source)?;

            info!("Loaded cache from {}", &cache.last_layout);
        },

        Err(e) => {
            warn!("Failed to parse cache file: {}. Ignoring.", e);
        }
    }

    Ok(())
}
