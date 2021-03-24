mod adapters;
mod core;
mod ui;

use crate::adapters::config::JsonConfig;
use crate::adapters::SimpleAudioPlayer;
use crate::adapters::SimpleAudioRepository;
use crate::core::config::Config;
use crate::core::Controller;

use crate::ui::ConsoleUI;
use crate::ui::RfidUI;

fn main() {
    env_logger::init();

    let config = JsonConfig::new(std::path::Path::new("yotibox.json"));
    let config2 = config.clone();
    let audio_repo_base_dir = std::path::PathBuf::from(config.audio_base_dir().unwrap());
    let audio_repo = SimpleAudioRepository::new(audio_repo_base_dir);
    let audio_repo2 = audio_repo.clone();
    let audio_player = SimpleAudioPlayer::new();
    let audio_player2 = audio_player.clone();

    let join_handle = std::thread::spawn(move || {
        let controller = Controller::new(&audio_repo, &audio_player);
        let mut ui = ConsoleUI::new(controller, &config);
        ui.run();
    });

    let join_handle2 = std::thread::spawn(move || {
        let controller = Controller::new(&audio_repo2, &audio_player2);
        let mut ui = RfidUI::new(controller, &config2);
        ui.run();
    });

    join_handle.join().unwrap();
    join_handle2.join().unwrap();
}
