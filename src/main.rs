#![feature(lazy_cell)]

mod audio;
mod image;
mod song;

#[cfg(feature = "miyoo")]
mod miyoo;

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use slint::Timer;

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

    let timer = Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_secs(1), {
        let app = app.as_weak();
        move || {
            let app = app.unwrap();
            let now_playing = app.global::<NowPlaying>();
            if now_playing.get_is_playing() {
                let song = now_playing.get_song();
                let progress = now_playing.get_progress();
                if progress >= song.duration {
                    info!("end song");
                } else {
                    now_playing.set_progress(progress + 1);
                }
            }
        }
    });

    app.global::<NowPlaying>().set_is_playing(true);
    app.global::<NowPlaying>().set_song(Song {
        path: song.path.to_string_lossy().as_ref().into(),
        title: song.title.as_deref().unwrap_or_default().into(),
        artist: song.artist.as_deref().unwrap_or_default().into(),
        album: song.album.as_deref().unwrap_or_default().into(),
        cover_art: song.cover_art(24)?,
        duration: song.duration.as_secs() as i32,
    });

    app.global::<Format>().on_format_time(|seconds: i32| {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{minutes:02}:{seconds:02}").into()
    });

    app.global::<MusicService>().on_load_song(|song| {
        info!("loaded: {:?}", song);
        let _ = audio::AUDIO
            .lock()
            .unwrap()
            .load(Path::new(song.path.as_str()));
    });

    app.global::<MusicService>().on_play(|| {
        let _ = audio::AUDIO.lock().unwrap().play();
    });

    app.global::<MusicService>().on_pause(|| {
        let _ = audio::AUDIO.lock().unwrap().pause();
    });

    info!("running event loop");
    app.run().unwrap();

    Ok(())
}
