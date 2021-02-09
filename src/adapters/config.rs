use crate::core::config::Config;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct JsonConfig {
    root: Value,
}

impl JsonConfig {
    pub fn new(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();

        JsonConfig {
            root: serde_json::from_str(&content).unwrap(),
        }
    }
}

impl Config for JsonConfig {
    fn rfid_spi_dev(&self) -> Option<&str> {
        self.root["rfid_spi_dev"].as_str()
    }

    fn rfid_poll_ms(&self) -> Option<u64> {
        self.root["rfid_poll_ms"].as_u64()
    }
}
