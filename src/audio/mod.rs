use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use log::info;
use slint::ComponentHandle;

use crate::{MainWindow, NowPlaying};

#[cfg(feature = "miyoo")]
mod oss;
#[cfg(feature = "miyoo")]
pub static AUDIO: LazyLock<oss::Oss> = LazyLock::new(|| oss::Oss::new());

#[cfg(feature = "simulator")]
mod rodio;
#[cfg(feature = "simulator")]
pub static AUDIO: LazyLock<rodio::Rodio> = LazyLock::new(|| rodio::Rodio::new().unwrap());

pub trait Audio {
    fn load(&self, path: &Path) -> Result<()>;
    fn play(&self) -> Result<()>;
    fn pause(&self) -> Result<()>;
    fn seek(&self, timestamp: i32) -> Result<()>;
}

pub fn setup(app: &MainWindow) {
    app.global::<NowPlaying>().set_is_playing(true);

    app.global::<NowPlaying>().on_load_song(|song| {
        let _ = AUDIO.load(Path::new(song.path.as_str()));
    });

    app.global::<NowPlaying>().on_play(|| {
        let _ = AUDIO.play();
    });

    app.global::<NowPlaying>().on_pause(|| {
        let _ = AUDIO.pause();
    });

    app.global::<NowPlaying>().on_seek(|duration| {
        let _ = AUDIO.seek(duration);
    });
}
