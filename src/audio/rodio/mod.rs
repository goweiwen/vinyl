use anyhow::Result;
use log::warn;
use rodio::{Decoder, OutputStream, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use crate::audio::Audio;

pub struct Rodio {
    sink: rodio::Sink,
}

impl Rodio {
    pub fn new() -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        Box::leak(Box::new(stream));
        let sink = rodio::Sink::try_new(&handle)?;

        Ok(Self { sink })
    }
}

impl Audio for Rodio {
    fn load(&self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let file = BufReader::new(file);
        let decoder = Decoder::new(file)?;
        self.sink.append(decoder);
        Ok(())
    }

    fn play(&self) -> Result<()> {
        self.sink.play();
        Ok(())
    }

    fn pause(&self) -> Result<()> {
        self.sink.pause();
        Ok(())
    }

    fn seek(&self, timestamp: i32) -> Result<()> {
        self.sink
            .try_seek(Duration::from_secs(timestamp as u64))
            .map_err(|e| warn!("Failed to seek: {}", e))
            .ok();
        Ok(())
    }
}
