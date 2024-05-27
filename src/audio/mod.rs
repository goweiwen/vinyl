use std::path::Path;

use anyhow::Result;

#[cfg(feature = "miyoo")]
mod oss;
#[cfg(feature = "miyoo")]
pub static AUDIO: oss::Oss = oss::Oss {};

#[cfg(not(feature = "miyoo"))]
mod rodio;
#[cfg(not(feature = "miyoo"))]
pub static AUDIO: rodio::Rodio = rodio::Rodio {};

pub trait Audio {
    fn play(&self, path: &Path) -> Result<()>;
}
