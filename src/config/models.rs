use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct WindowConfig {
    pub bg_colour: String
}
