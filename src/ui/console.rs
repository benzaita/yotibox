use crate::core::Command;

pub struct ConsoleUI {
}

impl ConsoleUI {
    pub fn new() -> Self {
        ConsoleUI { }
    }

    pub fn run(&mut self, tx: std::sync::mpsc::Sender<Command>) {
        loop {
            let action_index = self.prompt_for_action(&["Load 1", "Load 2", "Unload", "Reset"]);
            let maybe_command = match action_index {
                Some(0) => Some(Command::Load("1".into())),
                Some(1) => Some(Command::Load("2".into())),
                Some(2) => Some(Command::Unload),
                _ => None
            };

            maybe_command.map(|c| tx.send(c));
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
