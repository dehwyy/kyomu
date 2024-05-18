use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub name: String,
    pub funny_pastas: bool
}

impl Settings {
    fn to_json(&self) -> String {
        to_string_pretty(self).unwrap()
    }
}