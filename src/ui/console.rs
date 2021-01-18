use crate::core::Controller;

pub struct ConsoleUI<'a> {
    controller: Controller<'a>,
}

impl ConsoleUI<'_> {
    pub fn new(controller: Controller) -> ConsoleUI {
        ConsoleUI { controller }
    }

    pub fn run(&mut self) {
        loop {
            let action_index = self.prompt_for_action(&["Load 1", "Load 2", "Unload", "Reset"]);
            match action_index {
                Some(0) => self.controller.load("1"),
                Some(1) => self.controller.load("2"),
                Some(2) => self.controller.unload(),
                Some(3) => self.controller.reset(),
                _ => {}
            }
        }
    }

    fn prompt_for_action(&self, items: &[&str]) -> Option<usize> {
        dialoguer::Select::new()
            .with_prompt("Action")
            .items(items)
            .interact_opt()
            .unwrap()
    }
}