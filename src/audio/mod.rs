use std::path::Path;

use anyhow::Result;

#[cfg(feature = "miyoo")]
mod oss;

#[cfg(not(feature = "miyoo"))]
mod rodio;

trait Audio {
    fn play(path: &Path) -> Result<()>;
}
