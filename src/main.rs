#![feature(lazy_cell)]

mod audio;
mod song;

#[cfg(feature = "miyoo")]
mod miyoo;

use std::{
    borrow::BorrowMut,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::song::SongData;
use audio::Audio;

slint::include_modules!();

#[derive(Debug, Parser)]
#[command(name = "vinyl", version, about, long_about = None)]
#[command(bin_name = "vinyl")]
struct VinylCli {
    path: PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = VinylCli::parse();

    SimpleLogger::new()
        .with_level(
            #[cfg(debug_assertions)]
            if args.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
            #[cfg(not(debug_assertions))]
            if args.verbose {
                LevelFilter::Info
            } else {
                LevelFilter::Warn
            },
        )
        .init()
        .unwrap();

    run(&args.path)?;

    Ok(())
}

fn run(path: &Path) -> Result<()> {
    #[cfg(feature = "miyoo")]
    {
        slint::platform::set_platform(Box::new(miyoo::MyPlatform::new())).unwrap();
    }

    info!("initializing Vinyl...");
    let app = MainWindow::new().unwrap();

    let song = SongData::load(path.to_path_buf())?;

    app.set_model(Model {
        now_playing: NowPlaying {
            duration: 10.0,
            is_playing: false,
            progress: 0.0,
            song: Song {
                path: song.path.to_string_lossy().as_ref().into(),
                title: song.title.as_deref().unwrap_or_default().into(),
                artist: song.artist.as_deref().unwrap_or_default().into(),
                album: song.album.as_deref().unwrap_or_default().into(),
                cover_art: song.cover_art()?,
            },
        },
        songs: [].into(),
    });

    app.global::<MusicService>().on_load_song(|song| {
        info!("loaded: {:?}", song);
        audio::AUDIO
            .lock()
            .unwrap()
            .load(Path::new(song.path.as_str()));
    });

    app.global::<MusicService>().on_play(|| {
        audio::AUDIO.lock().unwrap().play();
    });

    app.global::<MusicService>().on_pause(|| {
        audio::AUDIO.lock().unwrap().pause();
    });

    info!("running event loop");
    app.run().unwrap();

    Ok(())
}
