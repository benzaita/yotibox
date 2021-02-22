use crate::core::config::Config;
use tinyjson::JsonValue;
use std::fs;
use std::path::Path;

pub struct JsonConfig {
    root: JsonValue,
}

impl JsonConfig {
    pub fn new(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();

        JsonConfig {
            root: content.parse().unwrap(),
        }
    }
}

impl Config for JsonConfig {
    fn rfid_spi_dev(&self) -> Option<&str> {
        match &self.root["rfid_spi_dev"] {
            JsonValue::String(s) => Some(&s),
            _ => None,
        }
    }

    fn rfid_poll_ms(&self) -> Option<u64> {
        match self.root["rfid_poll_ms"] {
            JsonValue::Number(n) => Some(n as u64),
            _ => None,
        }
    }
}
