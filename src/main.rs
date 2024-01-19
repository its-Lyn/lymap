
use config::{create::create_config, models::{WindowConfig, ButtonConfig}};

use device_query::{DeviceState, Keycode, DeviceQuery};

use env_logger::Builder;
use log::{error, warn, LevelFilter, info};

use macroquad::{miniquad, window::{next_frame, clear_background, request_new_screen_size}, text::load_ttf_font, input::{is_key_down, is_key_released}};

use native_dialog::FileDialog;
use ui::buttons;
use utils::{colour::hex_to_rgb, config_path::get_config_path};

mod window;
mod utils;
mod config;
mod ui;

#[macroquad::main("Lymap")]
async fn main() {
    // Initialise logger before everything else (meow)
    
    // Create custom init
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Info);

    builder.init();

    // Create lymap configs, aka CONFIG/lymap and lymap/layouts
    create_config();
    
    // Init
    let window_config: WindowConfig;
    match std::fs::read_to_string(&format!("{}/window_config.json", get_config_path())) {
        Ok(json) => {
            match serde_json::from_str::<WindowConfig>(&json) {
                Ok(config) => {
                    window_config = config;
                },

                Err(e) => {
                    error!("Failed to deserialise window config: {}", e);
                    std::process::exit(1);
                }
            } 
        },

        Err(e) => {
            error!("Failed to parse config file: {}", e);
            std::process::exit(1);
        }
    };

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

    let mut config_load = false;

    let device_state = DeviceState::new(); 
    
    // Load font
    let font = load_ttf_font("./assets/OpenSans-Regular.ttf").await.unwrap(); 

    loop {
        // Update
        let keys: Vec<Keycode> = device_state.get_keys();

        if is_key_down(miniquad::KeyCode::LeftControl) && is_key_down(miniquad::KeyCode::L) && !config_load {
            info!("Trying to load config folder...");
        
            let path = FileDialog::new()
                .set_location(&format!("{}/layouts", get_config_path()))
                .add_filter("JSON File", &["json"])
                .show_open_single_file();

            match path {
                Ok(val) => {
                    if let Err(e) = val {
                        error!("Failed to parse string: {}", e);
                        std::process::exit(1);
                    }
                    
                    match std::fs::read_to_string(val) {
                        Ok(source) => {
                            match serde_json::from_str(&source) {
                                Ok(config) => {
                                    btn_config = config;
                                },

                                Err(e) => {
                                    error!("Failed to deserialise config: {}", e);
                                    std::process::exit(1);
                                } 
                            }
                        },

                        Err(e) => {
                            error!("Failed to parse JSON source: {}", e);
                            std::process::exit(1);
                        }
                    }

                },

                Err(e) => {
                    error!("Failed to initialise file dialog: {}", e);
                    std::process::exit(1);
                }
            }

            config_load = true;
        } else if is_key_released(miniquad::KeyCode::LeftControl) || is_key_released(miniquad::KeyCode::L) {
            config_load = false;
        }

        // Draw
        clear_background(bg_colour);
        
        if let Some(config) = &btn_config { 
            for button in config.buttons.iter() { 
                buttons::draw_fmt(button, &font, &keys, &config.idle_colour, &config.active_colour);
            }
        }

        // Advance frame
        next_frame().await
    }
}
