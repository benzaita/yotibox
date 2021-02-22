use crate::core::AudioRepository;
use log::info;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub struct SimpleAudioRepository {
    base_dir: PathBuf,
    audio_extension: String,
}

impl SimpleAudioRepository {
    pub fn new(base_dir: PathBuf) -> SimpleAudioRepository {
        SimpleAudioRepository {
            base_dir,
            audio_extension: String::from("ogg"),
        }
    }

    fn dir_from_id(&self, id: &str) -> PathBuf {
        self.base_dir.join(Path::new(id))
    }

    fn first_audio_file_in_dir(&self, dir: &Path) -> Result<PathBuf> {
        let entries_or_errs = fs::read_dir(&dir)?;
        let paths_or_errs = entries_or_errs.map(|res| res.map(|item| item.path()));
        let paths: Vec<PathBuf> = paths_or_errs.collect::<io::Result<Vec<PathBuf>>>()?;
        let mut audio_files = paths
            .iter()
            .filter(|path| path.extension().and_then(OsStr::to_str) == Some(&self.audio_extension));

        let first_item = audio_files.next().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No audio files in directory: {}", dir.display()),
        ))?;

        Ok(first_item.clone())
    }
}

impl AudioRepository for SimpleAudioRepository {
    fn get_by_id(&self, id: &str) -> Result<File> {
        let dir = self.dir_from_id(id);
        let first_item_path = self.first_audio_file_in_dir(&dir)?;
        info!("Reading file {:?}", first_item_path);

        File::open(first_item_path)
    }

    fn create_stub_for_id(&self, id: &str) -> Result<()> {
        let dir = self.dir_from_id(id);
        std::fs::create_dir(&dir)?;
        let mut readme = File::create(dir.join("README.md"))?;
        readme.write_all(b"Place an audio file in this directory")?;
        Ok(())
    }
}
