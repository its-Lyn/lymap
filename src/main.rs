use config::{create::create_config, models::{WindowConfig, ButtonConfig}};

use device_query::{DeviceState, Keycode, DeviceQuery};

use env_logger::Builder;
use log::{error, warn, LevelFilter, info};

use macroquad::{miniquad, window::{next_frame, clear_background, request_new_screen_size}, text::{load_ttf_font, Font}, input::is_key_down};

use native_dialog::FileDialog;
use ui::buttons;
use utils::{colour::hex_to_rgb, config_path::get_config_path};

use crate::utils::config_path::get_assets_path;

mod utils;
mod config;
mod ui;
mod cache;

// Macro for easily handling errors in one line..
macro_rules! if_err {
    ($expression:expr, $message:tt) => {
        match $expression {
            Ok(ok) => ok,

            Err(e) => {
                error!("{}: {}", $message, e);
            
                std::process::exit(1);
            }
        }
    };
}

#[macroquad::main("Lymap")]
async fn main() {
    // Initialise logger before everything else (meow)
    
    // Create custom init
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Info);
    builder.format_timestamp_millis();

    builder.init();

    let lymap_version = "0.1.9-beta";
    let current_os = std::env::consts::OS;
    let current_arch = std::env::consts::ARCH;

    info!("Lymap version: {}", lymap_version);
    info!("Running on: {}", current_os);
    info!("CPU Arch: {}", current_arch);

    // Create lymap configs, aka <CONFIG>/lymap and lymap/layouts
    if_err!(create_config(), "Failed to create config");
    
    // Init 
    let window_path = &format!("{}/window_config.json", if_err!(get_config_path(), "Failed to fetch variable"));
    let window_source = if_err!(std::fs::read_to_string(window_path), "Failed to fetch window config json source");
    let window_config = if_err!(serde_json::from_str::<WindowConfig>(&window_source), "Failed to parse window config json");

    // Warn if either width or height are set without the other
    if window_config.width.is_some() && window_config.height.is_none() {
        warn!("Window width is set, but height isn't. Ignoring.");
    } else if window_config.width.is_none() && window_config.height.is_some() {
        warn!("Window width is not set, but height is. Ignoring.");
    }

    // Change window size if BOTH optional fields are set.
    if window_config.width.is_some() 
    && window_config.height.is_some() {
        request_new_screen_size(
            window_config.width.unwrap(), 
            window_config.height.unwrap()
        );
    }

    let bg_colour = hex_to_rgb(&window_config.bg_colour);
    let mut btn_config: Option<ButtonConfig> = Default::default();
    if_err!(cache::load::load(&mut btn_config), "Failed to load cache");

    let mut config_load = false;
    let mut config_reset = false;

    let device_state = DeviceState::new(); 
    
    // Load font
    let font: Font;
    match &window_config.font_path {
        Some(font_path) => {
            font = match load_ttf_font(font_path).await {
                Ok(font_ok) => {
                    info!("Loaded font {}", font_path);
                    font_ok
                },
        
                Err(e) => {
                    error!("Failed to load font: {}. Trying to fall back to default.", e);
        
                    if_err!(load_ttf_font(&format!("{}/Ubuntu-Regular.ttf", if_err!(get_assets_path(), "Failed to get asset path"))).await, "Failed to load default font.")
                }
            };
        },

        None => {
            info!("No font specified, using default font.");
            
            font = if_err!(load_ttf_font(&format!("{}/Ubuntu-Regular.ttf", if_err!(get_assets_path(), "Failed to get asset path"))).await, "Failed to load default font.");
        }
    };

    loop {
        // Update
        let keys: Vec<Keycode> = device_state.get_keys();

        if is_key_down(miniquad::KeyCode::LeftControl) && is_key_down(miniquad::KeyCode::L) {
            if !config_load {
                info!("Trying to load config folder...");
        
                let dialog = FileDialog::new()
                    .set_location(&format!("{}/layouts", if_err!(get_config_path(), "Failed to fetch variable")))
                    .add_filter("JSON File", &["json"])
                    .show_open_single_file();

                let possible_path = if_err!(dialog, "Failed to open file dialog"); 
            
                // User has the choice to either provide a path to the dialog or not.
                // this is not an error and should be handled accordingly. 
                if let Some(path) = possible_path {
                    info!("Path provided, continue.");
                    let config_source = if_err!(std::fs::read_to_string(&path), "Failed to fetch button config json");
            
                    match serde_json::from_str::<Option<ButtonConfig>>(&config_source) {
                        Ok(config) => { 
                            if config.is_some() {
                                info!("Loaded layout {}", &path.display());

                                // Transform Path to String and then write to cache
                                let str_path = if_err!(&path.to_str().ok_or("Invalid path provided"), "Failed convert path to string").to_string();
                                if_err!(cache::write::write(&str_path), "Failed to write cache");
                            }

                            btn_config = config; 
                        },

                        Err(e) => {
                            // This is not a fatal error, just skip loading the config.
                            warn!("Failed to parse button config: {}", e);
                        }
                    }
                }

                config_load = true;
            }   
        } else {
            config_load = false;
        }

        if is_key_down(miniquad::KeyCode::LeftControl) && is_key_down(miniquad::KeyCode::R) {
            if !config_reset {
                if btn_config.is_some() {
                    info!("Resetting button layout.");
                    btn_config = None; 
                    if_err!(cache::write::write(&""), "Failed to write cache");
                }

                config_reset = true;
            }
        } else  {
            config_reset = false;
        }

        // Draw
        clear_background(bg_colour);
        
        if let Some(config) = &btn_config { 
            for button in config.buttons.iter() { 
                buttons::draw_fmt(
                    button, 
                    &font, 
                    &keys, 
                    &config.idle_colour, 
                    &config.active_colour, 
                    window_config.font_size, 
                    &window_config.font_colour
                );
            }
        }

        // Advance frame
        next_frame().await
    }
}
