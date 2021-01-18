use log::error;
use std::error::Error;
use crate::core::Controller;
use serde_json::{Result, Value};

struct Command {
    typ: String,
    nfc_tag_id: String,
}

pub struct JsonCommandsOverStdinUI<'a> {
    controller: Controller<'a>,
}

impl JsonCommandsOverStdinUI<'_> {
    pub fn new(controller: Controller) -> JsonCommandsOverStdinUI {
        JsonCommandsOverStdinUI { controller }
    }

    pub fn run(&mut self) {
        use std::io::{self, BufRead};
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let maybe_command = JsonCommandsOverStdinUI::parse_json_command(line.unwrap().as_str());
            match maybe_command {
                Err(cause) => error!("Failed to parse action: {}", cause),
                Ok(action) => match action.typ.as_str() {
                    "load" => self.controller.load(&action.nfc_tag_id),
                    "unload" => self.controller.unload(),
                    "reset" => self.controller.reset(),
                    _ => {}
                },
            }
        }
    }

    fn parse_json_command(line: &str) -> std::result::Result<Command, Box<dyn Error>> {
        let serde_result: Result<Value> = serde_json::from_str(line);
        let value = serde_result?;
        let typ = value["type"].as_str().ok_or("string field 'type' missing")?.to_string();
        let nfc_tag_id = value["nfc_tag_id"].as_str().or(Some("")).unwrap().to_string();

        Ok(Command { typ, nfc_tag_id })
    }
}
