use log::error;
use std::error::Error;
use crate::core::Controller;
use serde_json::{Result, Value};

struct Action {
    typ: String,
    rfid_tag_id: String,
}

pub struct PhysicalUI<'a> {
    controller: Controller<'a>,
}

impl PhysicalUI<'_> {
    pub fn new(controller: Controller) -> PhysicalUI {
        PhysicalUI { controller }
    }

    pub fn run(&mut self) {
        loop {
            let maybe_action = PhysicalUI::poll();
            match maybe_action {
                Err(cause) => error!("Failed to poll: {}", cause),
                Ok(action) => match action.typ.as_str() {
                    "load" => self.controller.load(&action.rfid_tag_id),
                    "unload" => self.controller.unload(),
                    "reset" => self.controller.reset(),
                    _ => {}
                },
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    fn poll() -> std::result::Result<Action, Box<dyn Error>> {
        let poll_command = std::env::var("YOTIBOX_POLL_CMD")
            .expect("Environment variable YOTIBOX_POLL_CMD is not defined");
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(poll_command)
            .output()
            .expect("failed to execute poll command");
        assert!(output.status.success());

        let serde_result: Result<Value> = serde_json::from_slice(output.stdout.as_slice());
        let value = serde_result?;
        let typ = value["type"].as_str().ok_or("field 'type' missing")?.to_string();
        let rfid_tag_id = value["rfid_tag_id"].as_str().ok_or("field 'rfid_tag_id' missing")?.to_string();

        Ok(Action { typ, rfid_tag_id })
    }
}
