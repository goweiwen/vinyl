#![feature(lazy_cell)]

mod audio;
mod components;
mod image;
mod input;
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

slint::include_modules!();

#[derive(Debug, Parser)]
#[command(name = "vinyl", version, about, long_about = None)]
#[command(bin_name = "vinyl")]
struct VinylCli {
    path: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = VinylCli::parse();

    SimpleLogger::new()
        .with_level(
            #[cfg(debug_assertions)]
            if args.verbose {
                LevelFilter::Trace
            } else {
                LevelFilter::Debug
            },
            #[cfg(not(debug_assertions))]
            if args.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init()
        .unwrap();

    run(args.path.as_deref())?;

    Ok(())
}

fn run(path: Option<&Path>) -> Result<()> {
    #[cfg(feature = "miyoo")]
    {
        slint::platform::set_platform(Box::new(miyoo::MyPlatform::new())).unwrap();
    }

    info!("initializing Vinyl...");
    let app = MainWindow::new().unwrap();

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

    if let Some(path) = path {
        app.global::<NowPlaying>()
            .set_song((&SongData::load(path.to_path_buf()).unwrap()).into());
    }

    // app.global::<LibraryModel>().set_songs(
    //     [
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-01 Mr. Self Destruct.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-02 Piggy.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-03 Heresy.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-04 March of the Pigs.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-05 Closer.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-06 Ruiner.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/1-07 The Becoming.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-01 I Do Not Want This.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-02 Big Man With a Gun.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-03 A Warm Place.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-04 Eraser.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-05 Reptile.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-06 The Downward Spiral.m4a",
    //         "/mnt/d/Music/Nine Inch Nails/The Downward Spiral/2-07 Hurt.m4a",
    //     ]
    //     .iter()
    //     .map(|path| (&SongData::load(PathBuf::from(path)).unwrap()).into())
    //     .collect::<Vec<_>>()
    //     .as_slice()
    //     .into(),
    // );

    app.global::<Format>().on_format_time(|seconds: i32| {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{minutes:02}:{seconds:02}").into()
    });

    components::init(&app);

    info!("running event loop");
    app.run().unwrap();

    Ok(())
}
