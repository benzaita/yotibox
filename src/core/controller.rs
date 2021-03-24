use crate::core::AudioPlayer;
use crate::core::AudioRepository;
use log::{error, info};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
enum ControllerState {
    Idle,
    Playing,
}

#[derive(Clone)]
pub struct Controller<'a> {
    audio_repo: &'a dyn AudioRepository,
    audio_player: &'a dyn AudioPlayer,
    state: Arc<Mutex<ControllerState>>,
}

impl<'a> Controller<'a> {
    pub fn new(
        audio_repo: &'a dyn AudioRepository,
        audio_player: &'a dyn AudioPlayer,
    ) -> Controller<'a> {
        Controller {
            audio_repo,
            audio_player,
            state: Arc::new(Mutex::new(ControllerState::Idle)),
        }
    }

    pub fn load(&self, id: &str) {
        let mut guarded_state = self.state.lock().unwrap();
        match *guarded_state {
            ControllerState::Idle => {
                info!("Playing audio for id: {}", id);
                let maybe_audio_file = self.audio_repo.get_by_id(id);
                match maybe_audio_file {
                    Err(cause) => {
                        error!("Failed to load '{}': {}", id, cause);
                        self.audio_repo.create_stub_for_id(id).unwrap();
                    }
                    Ok(audio_file) => {
                        self.audio_player.play_file(audio_file);
                        *guarded_state = ControllerState::Playing;
                    }
                }
            }
            ControllerState::Playing => {
                info!(
                    "Ignoring request to load since alredy in {:?} state",
                    *guarded_state
                );
            }
        }
    }

    pub fn unload(&self) {
        let mut guarded_state = self.state.lock().unwrap();
        match *guarded_state {
            ControllerState::Idle => {
                info!(
                    "Ignoring request to unload since alredy in {:?} state",
                    *guarded_state
                );
            }
            ControllerState::Playing => {
                info!("Stopping playback");
                self.audio_player.stop();
                *guarded_state = ControllerState::Idle;
            }
        }
    }

    pub fn reset(&self) {
        self.unload();
    }
}
