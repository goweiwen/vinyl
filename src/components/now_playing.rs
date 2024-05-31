use std::path::Path;

use log::debug;
use slint::ComponentHandle;

use crate::audio::{Audio, AUDIO};
use crate::{MainWindow, NowPlaying};

pub fn init(app: &MainWindow) {
    let now_playing = app.global::<NowPlaying>();

    now_playing.set_is_playing(true);

    now_playing.on_load_song(|song| {
        debug!("load");
        let _ = AUDIO.load(Path::new(song.path.as_str()));
    });

    now_playing.on_play(|| {
        debug!("play");
        let _ = AUDIO.play();
    });

    now_playing.on_pause(|| {
        debug!("pause");
        let _ = AUDIO.pause();
    });

    now_playing.on_seek(|duration| {
        debug!("seek {}", duration);
        let _ = AUDIO.seek(duration);
    });
}
