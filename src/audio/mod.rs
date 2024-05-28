use std::path::Path;
use std::sync::{LazyLock, Mutex};

use anyhow::Result;

#[cfg(feature = "miyoo")]
mod oss;
#[cfg(feature = "miyoo")]
pub static AUDIO: oss::Oss = oss::Oss {};

#[cfg(not(feature = "miyoo"))]
mod rodio;
#[cfg(not(feature = "miyoo"))]
pub static AUDIO: LazyLock<Mutex<rodio::Rodio>> =
    LazyLock::new(|| Mutex::new(rodio::Rodio::new().unwrap()));

pub trait Audio {
    fn load(&mut self, path: &Path) -> Result<()>;
    fn play(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
}
