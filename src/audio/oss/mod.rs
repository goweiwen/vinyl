mod resampler;

use anyhow::{Context, Result};
use bytemuck::cast_slice;
use log::error;
use nix::ioctl_readwrite;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use crate::audio::oss::resampler::Resampler;
use crate::audio::Audio;

ioctl_readwrite!(dsp_speed, b'P', 2, i32);
ioctl_readwrite!(dsp_setfmt, b'P', 5, i32);
ioctl_readwrite!(dsp_channels, b'P', 6, i32);

static SAMPLE_RATE: u32 = 48000;
static BIT_RATE: i32 = 0x10;
static CHANNELS: i32 = 2;

enum Message {
    Load(PathBuf),
    Play,
    Pause,
    Seek(i32),
}

pub struct Oss {
    sender: kanal::Sender<Message>,
}

struct Track {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    resampler: Option<Resampler<i16>>,
    track_id: u32,
    samples: Vec<i16>,
}

impl Oss {
    pub fn new() -> Self {
        let (tx, rx) = kanal::unbounded();

        std::thread::spawn(move || {
            let mut dsp = OpenOptions::new().write(true).open("/dev/dsp").unwrap();

            unsafe {
                dsp_speed(dsp.as_raw_fd(), &mut (SAMPLE_RATE as i32 * 2)).unwrap(); // idk why music is playing at half speed. this is a hack
                dsp_setfmt(dsp.as_raw_fd(), &mut (BIT_RATE as i32)).unwrap();
                dsp_channels(dsp.as_raw_fd(), &mut (CHANNELS as i32)).unwrap();
            }

            let mut track: Option<Track> = None;
            let mut is_playing = true;
            loop {
                if let Some(msg) = rx.try_recv().unwrap() {
                    match msg {
                        Message::Load(path) => {
                            if let Ok(loaded_track) = load(&path) {
                                track = loaded_track;
                            }
                        }
                        Message::Play => {
                            is_playing = true;
                        }
                        Message::Pause => {
                            is_playing = false;
                        }
                        Message::Seek(duration) => {
                            if let Some(ref mut track) = track {
                                track
                                    .format
                                    .seek(
                                        SeekMode::Coarse,
                                        SeekTo::Time {
                                            time: Time::new(duration as u64, 0.0),
                                            track_id: None,
                                        },
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }

                if !is_playing {
                    continue;
                }

                let Some(Track {
                    ref mut format,
                    ref mut decoder,
                    ref mut resampler,
                    track_id,
                    ref mut samples,
                }) = track
                else {
                    continue;
                };

                if let Ok(Some(packet)) = format
                    .next_packet()
                    .map_err(|e| error!("Failed to load song: {}", e))
                {
                    // If the packet does not belong to the selected track, skip over it.
                    if packet.track_id() != track_id {
                        continue;
                    }

                    // Decode the packet into audio samples.
                    match decoder.decode(&packet) {
                        Ok(decoded) => {
                            let spec = decoded.spec();
                            if resampler.is_none() && spec.rate() != SAMPLE_RATE {
                                println!("Resampling {} Hz to {} Hz", spec.rate(), SAMPLE_RATE);
                                *resampler =
                                    Some(resampler::Resampler::new(spec, SAMPLE_RATE, 1024));
                            }

                            if let Some(resampler) = resampler {
                                resampler.resample(decoded, samples);
                            } else {
                                decoded.copy_to_vec_interleaved(samples);
                            }
                            dsp.write_all(cast_slice(&samples)).unwrap();
                        }
                        Err(Error::IoError(e)) => {
                            // The packet failed to decode due to an IO error, skip the packet.
                            println!("{e:?}");
                            continue;
                        }
                        Err(Error::DecodeError(e)) => {
                            // The packet failed to decode due to invalid data, skip the packet.
                            println!("{e:?}");
                            continue;
                        }
                        Err(err) => {
                            // An unrecoverable error occurred, halt decoding.
                            panic!("{}", err);
                        }
                    }
                }

                if let Some(resampler) = resampler {
                    resampler.flush(samples);
                    dsp.write_all(cast_slice(&samples)).unwrap();
                }
            }
        });

        Self { sender: tx }
    }
}

impl Audio for Oss {
    fn load(&self, path: &Path) -> Result<()> {
        self.sender
            .send(Message::Load(path.to_path_buf()))
            .context("Failed to send message")
    }

    fn play(&self) -> Result<()> {
        self.sender
            .send(Message::Play)
            .context("Failed to send message")
    }

    fn pause(&self) -> Result<()> {
        self.sender
            .send(Message::Pause)
            .context("Failed to send message")
    }

    fn seek(&self, timestamp: i32) -> Result<()> {
        self.sender
            .send(Message::Seek(timestamp))
            .context("Failed to send message")
    }
}

fn load(path: &Path) -> Result<Option<Track>> {
    let file = File::open(path)?;

    let mss_opts = MediaSourceStreamOptions::default();
    let mss = MediaSourceStream::new(Box::new(file), mss_opts);

    let mut hint = Hint::new();
    if let Some(ext) = path.extension() {
        hint.with_extension(&ext.to_string_lossy());
    };

    let meta_opts = MetadataOptions::default();
    let fmt_opts = FormatOptions::default();

    let format = symphonia::default::get_probe().format(&hint, mss, fmt_opts, meta_opts)?;

    let Some(track) = format.default_track() else {
        return Ok(None);
    };
    let track_id = track.id;

    let dec_opts = DecoderOptions::default();
    let decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

    Ok(Some(Track {
        format,
        decoder,
        resampler: None,
        track_id,
        samples: vec![],
    }))
}
