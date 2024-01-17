use config::create::create_config;
use macroquad::window::{next_frame, clear_background};
use utils::colour::hex_to_rgb;

mod window;
mod utils;
mod config;

#[macroquad::main("OwO")]
async fn main() {
    create_config();

    loop {
        clear_background(hex_to_rgb("#42a4ff"));
        next_frame().await
    }
}
