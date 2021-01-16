extern crate clap;

use std::env;

use clap::{App, Arg};

mod gameloader;

fn main() {
    let matches = App::new("Kakara Game Launcher").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Launches the Kakara game").
        arg(Arg::with_name("game").short("g").long("game").value_name("JAR_FILE").help("Takes the Kakara client").takes_value(true).required(true)).
        arg(Arg::with_name("engine").short("e").long("engine").value_name("JAR_FILE").help("Takes the Kakara Engine").takes_value(true).required(true)).
        arg(Arg::with_name("dir").short("w").long("working_dir").value_name("WORKING_DIRECTORY").help("What is the working directory for Kakara").takes_value(true).required(true)).
        arg(Arg::with_name("debug").short("d").long("debug").help("Specifies to load a debug game").takes_value(false).required(false)).
        get_matches();
    let gameJar = matches.value_of("game").unwrap_or("client.jar");
    let engineJar = matches.value_of("engine").unwrap_or("engine.jar");
    let workingDirectory = matches.value_of("dir").unwrap_or("test");
    let debugMode = matches.is_present("debug");
    println!("Loading Game jar: {}", gameJar);
    gameloader::load(gameJar, workingDirectory, engineJar, debugMode)
}
