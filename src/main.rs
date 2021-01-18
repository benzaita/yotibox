mod adapters;
mod core;
mod ui;

use crate::adapters::SimpleAudioPlayer;
use crate::adapters::SimpleAudioRepository;
use crate::core::Controller;
// use crate::ui::ConsoleUI;
use crate::ui::PhysicalUI;


fn main() {
    env_logger::init();

    let audio_repo_base_dir = std::env::current_dir().unwrap().join("resources");
    let audio_repo = SimpleAudioRepository::new(audio_repo_base_dir);
    let audio_player = SimpleAudioPlayer::new();
    let controller = Controller::new(&audio_repo, Box::new(audio_player));

    // ConsoleUI::new(controller).run();
    PhysicalUI::new(controller).run();
}
