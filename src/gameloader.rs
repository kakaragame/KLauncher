use std::{fs, thread};
use std::fs::File;
use std::io::BufReader;
use std::ops::Index;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use discord_rpc_client::Client;
use serde::{Deserialize, Serialize};

pub fn load(game: &str, dir: &str, engine: &str, debug: bool) {
    let mut home;
    // .kakara location is different depending on the OS.
    if cfg!(windows) {
        // Use %appdata% if in windows.
        home = Path::new(&format!("{}{}{}", std::env::var("USERPROFILE").unwrap(), "\\AppData", "\\Roaming")).join(".kakara");
    } else if cfg!(linux) {
        // Use /home/{user} if on linux. (This may work for mac. Currently untested.)
        home = Path::new(&std::env::var("HOME").unwrap()).join(".kakara");
    } else if cfg!(osx) {
        home = Path::new(&std::env::var("HOME").unwrap()).join("kakara");
    } else {
        // Else attempt to find the home environment variable.
        home = Path::new(&std::env::var("HOME").unwrap()).join(".kakara");
    }

    home = home.join("settings.yml");
    println!("Kakara Config {:?}", home.to_str());
    let ymlString = fs::read_to_string(home);
    let data: Settings = serde_yaml::from_str(&ymlString.unwrap()).unwrap();
    let mut java_command = Command::new(data.java);
    let testPath = Path::new(dir).join("test").join("test.yml");
    if testPath.exists() {
        let testFile = fs::read_to_string(testPath);
        let data: Data = serde_yaml::from_str(&testFile.unwrap()).unwrap();
        for x in data.launcher.arguments {
            java_command.arg(x);
        }
    }
    java_command.current_dir(dir)
        .arg("-cp").arg(engine).
        arg("-jar").arg(game).spawn();
    discordClient(dir)
}

fn discordClient(dir: &str) {
    //Ensure file was created
    thread::sleep(Duration::new(5, 0));
    let discord_file = Path::new(dir).join("discord.yml");

    let i = env!("DISCORD_KEY").parse().unwrap();
    let mut drpc = Client::new(i);
    drpc.start();
    println!("Starting Discord");
    while discord_file.exists() {
        print!("test");
        let testFile = fs::read_to_string(Path::new(dir).join("discord.yml"));
        let discord: Discord = serde_yaml::from_str(&testFile.unwrap()).unwrap();
        drpc.set_activity(|act| act.state(discord.current_task));
        thread::sleep(Duration::new(5, 0));
    }
}

#[derive(Deserialize)]
struct Settings {
    java: String
}

#[derive(Deserialize)]
struct Data {
    launcher: Launcher,
}

#[derive(Deserialize)]
struct Launcher {
    arguments: Vec<String>,
}

#[derive(Deserialize)]
struct Discord {
    current_task: String
}