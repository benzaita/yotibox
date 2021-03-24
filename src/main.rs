mod adapters;
mod core;
mod ui;

use crate::adapters::config::JsonConfig;
use crate::adapters::SimpleAudioPlayer;
use crate::adapters::SimpleAudioRepository;
use crate::core::config::Config;
use crate::core::Command;
use crate::core::Controller;
use crate::ui::ConsoleUI;
use crate::ui::RfidUI;
use std::path::PathBuf;
use std::sync::mpsc::channel;

fn main() {
    env_logger::init();

    let config = JsonConfig::new(std::path::Path::new("yotibox.json"));
    let audio_repo_base_dir: PathBuf = std::path::PathBuf::from(config.audio_base_dir().unwrap());
    let audio_repo = SimpleAudioRepository::new(audio_repo_base_dir);
    let audio_player = SimpleAudioPlayer::new();
    let controller = Controller::new(&audio_repo, &audio_player);

    let (tx, rx) = channel::<Command>();

    {
        let tx = tx.clone();

        std::thread::spawn(move || {
            let mut ui = ConsoleUI::new();
            ui.run(tx);
        });
    }

    {
        let tx = tx.clone();
        let config = config.clone();

        std::thread::spawn(move || {
            let mut ui = RfidUI::new(&config);
            ui.run(tx);
        });
    }

    loop {
        let command = rx.recv().unwrap();
        println!("Recv command: {:?}", command);
        match command {
            Command::Load(id) => controller.load(&id),
            Command::Unload => controller.unload(),
        }
    }
}
