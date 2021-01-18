use log::info;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use crate::core::AudioRepository;

pub struct SimpleAudioRepository {
    base_dir: PathBuf,
}

impl SimpleAudioRepository {
    pub fn new(base_dir: PathBuf) -> SimpleAudioRepository {
        SimpleAudioRepository { base_dir }
    }
}

impl AudioRepository for SimpleAudioRepository {
    fn get_by_id(&self, id: &str) -> Result<File> {
        let file_path = self.base_dir.join(Path::new(id)).with_extension("ogg");
        info!("Reading file {}", &file_path.to_str().unwrap());

        File::open(file_path)
    }
}
