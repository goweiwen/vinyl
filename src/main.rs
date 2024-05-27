#![feature(lazy_cell)]

mod audio;

#[cfg(feature = "miyoo")]
mod miyoo;

use anyhow::Result;
use clap::Parser;

slint::include_modules!();

#[derive(Debug, Parser)]
#[command(name = "vinyl", version, about, long_about = None)]
#[command(bin_name = "vinyl")]
struct VinylCli {}

fn main() -> Result<()> {
    let _ = VinylCli::parse();

    run()?;

    Ok(())
}

fn run() -> Result<()> {
    #[cfg(feature = "miyoo")]
    {
        slint::platform::set_platform(Box::new(miyoo::MyPlatform::new())).unwrap();
    }

    MainWindow::new().unwrap().run().unwrap();

    Ok(())
}
