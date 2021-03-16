use std::fs::File;
use std::io::BufReader;
use crate::core::AudioPlayer;

pub struct SimpleAudioPlayer {
    sink: rodio::Sink,
    device: rodio::Device,
}

impl SimpleAudioPlayer {
    pub fn new() -> SimpleAudioPlayer {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        SimpleAudioPlayer { device, sink }
    }
}

impl AudioPlayer for SimpleAudioPlayer {
    fn play_file(&self, file: File) {
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        // Why overwrite? It seems like Sink.append() after Sink.stop() does not play any sound
        // TODO this requires `&mut self`
        // self.sink = rodio::Sink::new(&self.device);

        self.sink.append(source);
    }

    fn stop(&self) {
        self.sink.stop();
    }
}
