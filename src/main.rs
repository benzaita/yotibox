mod adapters;
mod core;
mod ui;

use crate::adapters::config::JsonConfig;
use crate::adapters::SimpleAudioPlayer;
use crate::adapters::SimpleAudioRepository;
use crate::core::config::Config;
use crate::core::Controller;

#[cfg(feature = "ui_console")]
use crate::ui::ConsoleUI as UI;

#[cfg(feature = "ui_rfid")]
use crate::ui::RfidUI as UI;

fn main() {
    env_logger::init();

    let config = JsonConfig::new(std::path::Path::new("yotibox.json"));
    let audio_repo_base_dir = std::path::PathBuf::from(config.audio_base_dir().unwrap());
    let audio_repo = SimpleAudioRepository::new(audio_repo_base_dir);
    let audio_player = SimpleAudioPlayer::new();

    let join_handle = std::thread::spawn(move || {
        let controller = Controller::new(&audio_repo, &audio_player);
        let mut ui = UI::new(controller, &config);
        ui.run();
    });

    join_handle.join().unwrap();
}
