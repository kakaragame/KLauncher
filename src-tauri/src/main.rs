#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::installer::install;
use clap::{App, Arg};
use crate::profile::GameProfile;
use std::path::Path;

mod cmd;
mod jre;
mod downloader;
mod gameloader;
mod installer;
mod kconfig;
mod osspec;
mod utils;
mod test;
mod jenkins;
mod profile;
mod error;
mod account;

fn main() {
    if !installer::is_installed() {
        println!("Installing game");
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        runtime.block_on(install());
    }
    let matches = App::new("Kakara Game Launcher").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Launches the Kakara game").
        arg(Arg::with_name("game").short("g").long("game").value_name("JAR_FILE").help("Takes the Kakara client").takes_value(true).required(false)).
        arg(Arg::with_name("engine").short("e").long("engine").value_name("JAR_FILE").help("Takes the Kakara Engine").takes_value(true).required(false)).
        arg(Arg::with_name("dir").short("w").long("working_dir").value_name("WORKING_DIRECTORY").help("What is the working directory for Kakara").default_value("{KAKARA_HOME}/game").takes_value(true).required(false)).
        arg(Arg::with_name("test_mode").short("t").long("test_mode").takes_value(false).required(false)).
        get_matches();
    if matches.is_present("game") && matches.is_present("engine")  {
        let mut profile = GameProfile::new(String::from("tmp"), matches.value_of("game").unwrap().parse().unwrap(),
                                           matches.value_of("engine").unwrap().parse().unwrap(),
                                           matches.is_present("test_mode"));
        profile.set_game_directory(matches.value_of("dir").unwrap().parse().unwrap());
        launch(profile);
        return;
    }
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => {
                    Err(e.to_string())
                }
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        MyCustomCommand { argument } => {
                            //  your command code
                            println!("{}", argument);
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}

fn launch(profile: GameProfile) {
    println!("Launching Game: {}",&profile.name);
    let game_jar: String;
    if profile.version.starts_with("jenkins") {
        let version = profile.version.clone();
        let split = version.split(":");
        let vec = split.collect::<Vec<&str>>();

        let branch = vec.get(1).unwrap();
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let s = runtime.block_on(jenkins::download_game(branch));
        game_jar = s.unwrap();
    } else {
        panic!("use of unimplemented start mode");
    }
    let engine_jar: String;

    let working_directory = profile.game_directory.clone();
    let working_directory_path = Path::new(&working_directory);
    if profile.test_mode {
        if !test::is_installed(&working_directory_path) {
            test::install(&working_directory_path);
        }
    }
    if profile.engine_version.starts_with("jenkins") {
        let split = profile.engine_version.split(":");
        let vec = split.collect::<Vec<&str>>();

        let branch = vec.get(1).unwrap();
        let result = jenkins::get_branch_url(branch);
        let job = result.unwrap();
        let engine1 = jenkins::get_build_url(job);
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let s = runtime.block_on(jenkins::download_engine_jar(&*engine1));
        if s == "" {
            // If the engine version was not found.
            println!("[ERROR] Unable to download engine version. Please provide an engine build with --engine");
            return;
        }
        engine_jar = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine").join(s).to_str().unwrap().to_string();
    } else {
        panic!("use of unimplemented start mode");
    }

    println!("Loading Game jar: {}", game_jar);
    gameloader::load(game_jar.as_str(), &working_directory, engine_jar)
}