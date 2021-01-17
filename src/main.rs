extern crate clap;

use std::borrow::{Borrow, BorrowMut};
use std::path::Path;
use std::thread;

use clap::{App, Arg};
use futures::executor::block_on;
use futures::future::Ready;
use futures::task::{Context, RawWakerVTable, Waker};
use futures::{future, Future};
mod gameloader;
mod engine;
 fn main() {
    let matches = App::new("Kakara Game Launcher").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Launches the Kakara game").
        arg(Arg::with_name("game").short("g").long("game").value_name("JAR_FILE").help("Takes the Kakara client").takes_value(true).required(true)).
        arg(Arg::with_name("engine").short("e").long("engine").value_name("JAR_FILE").help("Takes the Kakara Engine").takes_value(true).required(false)).
        arg(Arg::with_name("dir").short("w").long("working_dir").value_name("WORKING_DIRECTORY").help("What is the working directory for Kakara").takes_value(true).required(true)).
        get_matches();
    let game_jar = matches.value_of("game").unwrap_or("client.jar");
    let mut engine_jar: String = String::new();
    if matches.is_present("engine") {
        engine_jar = matches.value_of("engine").unwrap_or("engine.jar").parse().unwrap();
    } else {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let s = runtime.block_on(engine::downloadLatestBuild());
        engine_jar = Path::new("engine").join(s).to_str().unwrap().parse().unwrap();
        println!("{}", engine_jar);
    }
    let working_directory = matches.value_of("dir").unwrap_or("test");
    println!("Loading Game jar: {}", game_jar);
    gameloader::load(game_jar, working_directory, engine_jar)
}

