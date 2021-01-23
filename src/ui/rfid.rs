use crate::core::Controller;
use crate::adapters::rfid::RC522RfidController;
use crate::adapters::rfid::RfidController;

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
}

impl RfidUI<'_> {
    pub fn new(controller: Controller) -> RfidUI {
        RfidUI {
            controller,
            state: State::TagNotPresent,
            rfid_controller: Box::new(RC522RfidController { dev: "/dev/spi1" }),
        }
    }

    pub fn run(&mut self) {
        loop {
            let maybe_tag = self.rfid_controller.poll_for_tag().unwrap();
            let (next_state, maybe_command) = match maybe_tag {
                None => self.handle_no_tag(),
                Some(tag_id) => self.handle_tag_present(&tag_id),
            };

            self.state = next_state;

            if let Some(command) = maybe_command {
                match command {
                    Command::Load(nfc_tag_id) => self.controller.load(&nfc_tag_id),
                    Command::Unload => self.controller.unload(),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
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
