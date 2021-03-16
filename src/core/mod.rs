mod controller;
pub mod config;

use std::fs::File;
use std::io::Result;

pub use controller::Controller;

pub trait AudioRepository {
    fn get_by_id(&self, id: &str) -> Result<File>;
    fn create_stub_for_id(&self, id: &str) -> Result<()>;
}

pub trait AudioPlayer {
    fn play_file(&self, file: File);
    fn stop(&self);
}
