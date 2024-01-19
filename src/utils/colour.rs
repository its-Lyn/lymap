use macroquad::prelude::Color;

pub fn hex_to_rgb(hex: &str) -> Color {
    // TODO: Handle

    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

    Color::from_rgba(r, g, b, 255)
}
