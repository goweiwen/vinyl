use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;

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
