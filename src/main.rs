use log::info;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let audio_repo_base_dir = std::env::current_dir().unwrap().join("resources");
    let audio_repo = SimpleAudioRepository::new(audio_repo_base_dir);
    let audio_player = SimpleAudioPlayer::new();
    let mut controller = Controller::new(&audio_repo, Box::new(audio_player));

    loop {
        let action_index = prompt_for_action(&["Load 1", "Load 2", "Unload", "Reset"]);
        match action_index {
            Some(0) => controller.load("1"),
            Some(1) => controller.load("2"),
            Some(2) => controller.unload(),
            Some(3) => controller.reset(),
            _ => {}
        }
    }
}

fn prompt_for_action(items: &[&str]) -> Option<usize> {
    dialoguer::Select::new()
        .with_prompt("Action")
        .items(items)
        .interact_opt()
        .unwrap()
}

trait AudioRepository {
    fn get_by_id(&self, id: &str) -> File;
}

struct SimpleAudioRepository {
    base_dir: PathBuf,
}

impl SimpleAudioRepository {
    fn new(base_dir: PathBuf) -> SimpleAudioRepository {
        SimpleAudioRepository { base_dir }
    }
}

impl AudioRepository for SimpleAudioRepository {
    fn get_by_id(&self, id: &str) -> File {
        let file_path = self.base_dir.join(Path::new(id)).with_extension("ogg");
        info!("Reading file {}", &file_path.to_str().unwrap());

        File::open(file_path).unwrap()
    }
}

trait AudioPlayer {
    fn play_file(&mut self, file: File);
    fn stop(&self);
}

struct SimpleAudioPlayer {
    sink: rodio::Sink,
    device: rodio::Device,
}

impl SimpleAudioPlayer {
    fn new() -> SimpleAudioPlayer {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        SimpleAudioPlayer { device, sink }
    }
}

impl AudioPlayer for SimpleAudioPlayer {
    fn play_file(&mut self, file: File) {
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        // Why overwrite? It seems like Sink.append() after Sink.stop() does not play any sound
        self.sink = rodio::Sink::new(&self.device);

        self.sink.append(source);
    }

    fn stop(&self) {
        self.sink.stop();
    }
}

#[derive(Debug)]
enum ControllerState {
    Idle,
    Playing,
}

struct Controller<'a> {
    audio_repo: &'a dyn AudioRepository,
    audio_player: Box<dyn AudioPlayer>,
    state: ControllerState,
}

impl<'a> Controller<'a> {
    fn new(audio_repo: &'a dyn AudioRepository, audio_player: Box<dyn AudioPlayer>) -> Controller {
        Controller {
            audio_repo,
            audio_player,
            state: ControllerState::Idle,
        }
    }

    fn load(&mut self, id: &str) {
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

    fn unload(&mut self) {
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

    fn reset(&mut self) {
        self.unload();
    }
}
