mod controller;

use std::fs::File;

pub use controller::Controller;

pub trait AudioRepository {
    fn get_by_id(&self, id: &str) -> File;
}

pub trait AudioPlayer {
    fn play_file(&mut self, file: File);
    fn stop(&self);
}
