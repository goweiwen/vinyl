use anyhow::Result;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::audio::Audio;

pub struct Rodio {
    sink: rodio::Sink,
}

impl Rodio {
    pub fn new() -> Result<Self> {
        // Get an output stream handle to the default physical sound device
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;
        sink.pause();

        Ok(Self { sink })
    }
}

impl Audio for Rodio {
    fn load(&mut self, path: &Path) -> Result<()> {
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path)?);
        // Decode that sound file into a source
        let decoder = Decoder::new(file)?;
        self.sink.append(decoder);

        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        self.sink.play();
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.sink.pause();
        Ok(())
    }
}
