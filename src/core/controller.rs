use log::info;
use crate::core::AudioPlayer;
use crate::core::AudioRepository;

#[derive(Debug)]
enum ControllerState {
    Idle,
    Playing,
}

pub struct Controller<'a> {
    audio_repo: &'a dyn AudioRepository,
    audio_player: Box<dyn AudioPlayer>,
    state: ControllerState,
}

impl<'a> Controller<'a> {
    pub fn new(audio_repo: &'a dyn AudioRepository, audio_player: Box<dyn AudioPlayer>) -> Controller {
        Controller {
            audio_repo,
            audio_player,
            state: ControllerState::Idle,
        }
    }

    pub fn load(&mut self, id: &str) {
        match self.state {
            ControllerState::Idle => {
                let audio_file = self.audio_repo.get_by_id(id);
                self.audio_player.play_file(audio_file);
                self.state = ControllerState::Playing;
            }
            ControllerState::Playing => {
                info!(
                    "Ignoring request to load since alredy in {:?} state",
                    self.state
                );
            }
        }
    }

    pub fn unload(&mut self) {
        match self.state {
            ControllerState::Idle => {
                info!(
                    "Ignoring request to load since alredy in {:?} state",
                    self.state
                );
            }
            ControllerState::Playing => {
                self.audio_player.stop();
                self.state = ControllerState::Idle;
            }
        }
    }

    pub fn reset(&mut self) {
        self.unload();
    }
}