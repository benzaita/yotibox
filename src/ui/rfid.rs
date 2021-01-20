use crate::core::Controller;
use log::error;
use std::io::Result;

struct Command {
    typ: String,
    nfc_tag_id: String,
}

enum State {
    TagNotPresent,
    TagPresent,
}

pub struct RfidUI<'a> {
    controller: Controller<'a>,
    state: State,
}

impl RfidUI<'_> {
    pub fn new(controller: Controller) -> RfidUI {
        RfidUI {
            controller,
            state: State::TagNotPresent,
        }
    }

    pub fn run(&mut self) {
        loop {
            let maybe_command = self.wait_for_command();

            match maybe_command {
                Err(cause) => error!("Failed to parse action: {}", cause),
                Ok(action) => match action.typ.as_str() {
                    "load" => self.controller.load(&action.nfc_tag_id),
                    "unload" => self.controller.unload(),
                    _ => {}
                },
            }
        }
    }

    fn wait_for_command(&mut self) -> Result<Command> {
        loop {
            let maybe_tag = poll_for_rfid_tag();
            let maybe_command = match maybe_tag {
                None => self.handle_no_tag(),
                Some(tag_id) => self.handle_tag_present(tag_id),
            };

            match maybe_command {
                Some(command) => return Ok(command),
                None => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }
    }

    fn handle_no_tag(&mut self) -> Option<Command> {
        let maybe_command = match self.state {
            State::TagNotPresent => None,
            State::TagPresent => Some(Command {
                typ: "unload".to_string(),
                nfc_tag_id: "".to_string(),
            }),
        };

        self.state = State::TagNotPresent;
        maybe_command
    }

    fn handle_tag_present(&mut self, tag_id: &str) -> Option<Command> {
        let maybe_command = match self.state {
            State::TagNotPresent => Some(Command {
                typ: "load".to_string(),
                nfc_tag_id: tag_id.to_string(),
            }),
            State::TagPresent => None,
        };

        self.state = State::TagPresent;
        maybe_command
    }
}
