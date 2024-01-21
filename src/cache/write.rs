use std::error::Error;

use crate::utils::config_path::get_config_path;

use super::model::Cache;

pub fn write(layout_path: &str) -> Result<(), Box<dyn Error>> {
    let cache = Cache {
        last_layout: layout_path.to_string()
    };

    let cache_path = format!("{}/cache/cache.json", get_config_path()?);
    let cache_file = std::fs::File::create(&cache_path)?;
    
    serde_json::to_writer_pretty(cache_file, &cache)?;

    Ok(())
}
