#![feature(lazy_cell)]

mod audio;

#[cfg(feature = "miyoo")]
mod miyoo;

use anyhow::Result;
use clap::Parser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

slint::include_modules!();

#[derive(Debug, Parser)]
#[command(name = "vinyl", version, about, long_about = None)]
#[command(bin_name = "vinyl")]
struct VinylCli {
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
