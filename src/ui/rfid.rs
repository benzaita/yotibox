use log::{error};
use crate::adapters::rfid::RC522RfidController;
use crate::adapters::rfid::RfidController;
use crate::core::config::Config;
use crate::core::Controller;

enum Command {
    Load(String),
    Unload,
}

enum State {
    TagNotPresent,
    TagPresent,
}

pub struct RfidUI<'a> {
    controller: Controller<'a>,
    state: State,
    rfid_controller: Box<dyn RfidController>,
    config: &'a dyn Config,
}

impl Drop for RfidUI<'_> {
    fn drop(&mut self) {
        self.rfid_controller.cleanup().unwrap();
    }
}

impl<'a> RfidUI<'a> {
    pub fn new(controller: Controller<'a>, config: &'a dyn Config) -> RfidUI<'a> {
        RfidUI {
            controller,
            state: State::TagNotPresent,
            rfid_controller: Box::new(RC522RfidController::new(config.rfid_spi_dev().unwrap())),
            config,
        }
    }

    pub fn run(&mut self) {
        self.rfid_controller.init().unwrap();
        loop {
            let maybe_tag = match self.state {
                State::TagNotPresent => self.rfid_controller.read_id_from_idle_picc(),
                State::TagPresent => self.rfid_controller.read_if_from_halted_picc(),
            };

            let (next_state, maybe_command) = match maybe_tag {
                Ok(None) => self.handle_no_tag(),
                Ok(Some(tag_id)) => self.handle_tag_present(&tag_id),
                Err(e) => {
                    error!("Encountered error: {:?}", e);
                    self.handle_no_tag()
                },
            };

            self.state = next_state;

            if let Some(command) = maybe_command {
                match command {
                    Command::Load(nfc_tag_id) => self.controller.load(&nfc_tag_id),
                    Command::Unload => self.controller.unload(),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(
                self.config.rfid_poll_ms().unwrap_or(1000),
            ));
        }
    }

    fn handle_no_tag(&self) -> (State, Option<Command>) {
        let maybe_command = match self.state {
            State::TagNotPresent => None,
            State::TagPresent => Some(Command::Unload),
        };

        (State::TagNotPresent, maybe_command)
    }

    fn handle_tag_present(&self, tag_id: &str) -> (State, Option<Command>) {
        let maybe_command = match self.state {
            State::TagNotPresent => Some(Command::Load(String::from(tag_id))),
            State::TagPresent => None,
        };

        (State::TagPresent, maybe_command)
    }
}
