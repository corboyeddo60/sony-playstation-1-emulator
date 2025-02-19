use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;

pub struct AudioPlayer {
    stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream = OutputStream::try_default().unwrap();
        AudioPlayer { stream }
    }

    pub fn play_sound(&self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let source = Decoder::new_wav(file).unwrap();
        self.stream.play_raw(source.convert_samples()).unwrap();
    }
}