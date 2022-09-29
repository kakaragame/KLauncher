#![allow(unused, dead_code)]
#![deny(deprecated, deprecated_in_future)]

use clap::Parser;
use std::env::current_dir;
use std::path::{Path, PathBuf};

use crate::error::LauncherError;

use crate::installer::install;

mod downloader;
pub mod error;
mod gameloader;
mod installer;
mod jenkins;
mod jre;
mod osspec;
mod settings;
mod test;
mod utils;

#[derive(Parser)]
#[command(about = "Launch Kakara", version, author)]
pub struct KLauncherCommand {
    #[arg(short, long, default_value = "jenkins:master")]
    pub engine: String,
    #[arg(short, long, default_value = "jenkins:master")]
    pub game: String,
    #[arg(short, long)]
    pub working_dir: Option<PathBuf>,
    #[arg(short, long)]
    pub test_mode: bool,
}

fn main() -> Result<(), LauncherError> {
    let current_exe = std::env::current_dir().expect("current exe");

    let _logger_config = include_str!("log.json");

    if !installer::is_installed() {
        println!("Installing game");
        install();
    }
    let command: KLauncherCommand = KLauncherCommand::parse();
    let game_jar = if command.game.starts_with("jenkins") {
        let split = command.game.split(':');
        let vec = split.collect::<Vec<&str>>();

        let branch = vec.get(1).unwrap();
        let s = jenkins::download_game(branch);
        match s {
            Ok(ok) => current_exe.join("game").join(ok),
            Err(_) => {
                return Err(LauncherError::Custom("Unable to download game".to_string()));
            }
        }
    } else {
        PathBuf::from(command.game)
    };
    let working_directory_path = command
        .working_dir
        .unwrap_or(current_dir().unwrap().join("test"));
    if command.test_mode {
        test::install(&working_directory_path);
    }
    let engine_jar = if command.engine.starts_with("jenkins") {
        let split = command.engine.split(':');
        let vec = split.collect::<Vec<&str>>();

        let branch = vec.get(1).unwrap();
        match jenkins::download_engine_jar(branch) {
            Ok(ok) => current_exe.join("engine").join(ok),
            Err(_) => {
                return Err(LauncherError::Custom("[ERROR] Unable to download engine version. Please provide an engine build with --engine".to_string()));
            }
        }
    } else {
        Path::new(&command.engine).to_path_buf()
    };

    gameloader::load(game_jar, working_directory_path, engine_jar);
    Ok(())
}
