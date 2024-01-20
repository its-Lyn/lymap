use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct WindowConfig {
    pub bg_colour: String,

    pub font_path: String,
    pub font_size: u16,
    pub font_colour: String,

    pub width: Option<f32>,
    pub height: Option<f32>
}

#[derive(Deserialize, Serialize)]
pub struct Button {
    pub x: f32,
    pub y: f32,

    pub text: String,
    pub key: String,

    pub width: f32,
    pub height: f32
}

#[derive(Deserialize, Serialize)]
pub struct ButtonConfig {
    pub idle_colour: String,
    pub active_colour: String,

    pub buttons: Vec<Button>
}
