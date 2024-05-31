pub mod now_playing;

use crate::MainWindow;

pub fn init(app: &MainWindow) {
    now_playing::init(app);
}
