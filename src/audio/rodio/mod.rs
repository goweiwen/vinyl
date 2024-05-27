use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::audio::Audio;

pub struct Rodio {}

impl Audio for Rodio {
    fn play(&self, path: &Path) -> Result<()> {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default()?;
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path)?);
        // Decode that sound file into a source
        let source = Decoder::new(file)?;
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples())?;

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(5));

        Ok(())
    }
}
