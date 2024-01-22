use std::str::FromStr;

use device_query::Keycode;
use log::error;
use macroquad::{text::{Font, draw_text_ex, measure_text, TextParams}, shapes::draw_rectangle};

use crate::{config::models::Button, utils::colour::hex_to_rgb};

pub fn draw_fmt(button: &Button, font: &Font, keys: &Vec<Keycode>, idle: &str, active: &str, fs: u16, fc: &str) { 
    let btn_hex;
    match Keycode::from_str(&button.key) {
        Ok(keycode) => {
            if keys.contains(&keycode) {
                btn_hex = active;
            } else {
                btn_hex = idle;
            }
        },

        Err(e) => {
            error!("Failed to parse button key: {}", e);
            std::process::exit(1);
        }
    }

    let text_dimensions = measure_text(&button.text, Some(font), fs, 1.0);

    draw_rectangle(button.x, button.y, button.width, button.height, hex_to_rgb(btn_hex));
    draw_text_ex(
        &button.text,
        button.x + (button.width - text_dimensions.width) / 2.0,
        button.y + (button.height - text_dimensions.height) / 2.0 + text_dimensions.offset_y,
        TextParams {
            font: Some(font),
            font_size: fs,
            font_scale: 1.0,
            color: hex_to_rgb(&fc),
            ..Default::default()
        }
    );
}
