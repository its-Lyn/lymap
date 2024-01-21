use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Cache {
    pub last_layout: String
}
