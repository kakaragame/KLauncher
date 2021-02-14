extern crate clap;

use std::path::Path;

use clap::{App, Arg};

use crate::installer::install;

mod gameloader;
mod osspec;
mod jenkins;
mod downloader;
mod jre;
mod utils;
mod installer;
mod settings;

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
        arg(Arg::with_name("dir").short("w").long("working_dir").value_name("WORKING_DIRECTORY").help("What is the working directory for Kakara").takes_value(true).required(false)).
        get_matches();
    let x = matches.value_of("game").unwrap_or("client.jar");
    let game_jar: String;
    if x.starts_with("jenkins") {
        let split = x.split(":");
        let vec = split.collect::<Vec<&str>>();

        let branch = vec.get(1).unwrap();
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let s = runtime.block_on(jenkins::download_game(branch));
        game_jar = s.unwrap();
    } else {
        game_jar = String::from(x);
    }
    let engine_jar: String;

    let working_directory = matches.value_of("dir").unwrap_or("test");
    if matches.is_present("engine") {
        let engine_string: String = matches.value_of("engine").unwrap_or("engine.jar").parse().unwrap();
        if engine_string.starts_with("jenkins") {
            let split = engine_string.split(":");
            let vec = split.collect::<Vec<&str>>();

            let branch = vec.get(1).unwrap();
            let result = jenkins::getBranchURL(branch);
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
            engine_jar = engine_string;
        }
    } else {
        let result = jenkins::getBranchURL("master");
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
        println!("{}", engine_jar);
    }
    println!("Loading Game jar: {}", game_jar);
    gameloader::load(game_jar.as_str(), working_directory, engine_jar)
}
