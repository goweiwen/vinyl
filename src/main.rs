#![feature(lazy_cell)]

mod audio;

#[cfg(feature = "miyoo")]
mod miyoo;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

slint::include_modules!();

#[derive(Debug, Parser)]
#[command(name = "vinyl")]
#[command(bin_name = "vinyl")]
struct VinylCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
#[command(version, about, long_about = None)]
enum Commands {
    Run,
    #[command(arg_required_else_help = true)]
    Play {
        path: PathBuf,
    },
    #[command(arg_required_else_help = true)]
    Raw {
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = VinylCli::parse();

    match args.command {
        Commands::Run => {
            run()?;
        }
        Commands::Play { path } => {
            play(&path)?;
        }
        Commands::Raw { path } => {
            raw(&path)?;
        }
    }

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

fn raw(path: &Path) -> Result<()> {
    audio::play_raw(path)?;
    Ok(())
}

fn play(path: &Path) -> Result<()> {
    audio::play(path)?;
    Ok(())
}
