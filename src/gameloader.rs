use std::{fs, thread};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use discord_rpc_client::Client;
use serde::Deserialize;

use crate::osspec;
use discord_rpc_client::models::Activity;

pub fn load(game: &str, dir: &str, engine: String) {
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
    let yml_string = fs::read_to_string(home);
    let data: Settings = serde_yaml::from_str(&yml_string.unwrap()).unwrap();
    let mut java_command = Command::new(data.java);
    let test_path = Path::new(dir).join("test").join("test.yml");
    if test_path.exists() {
        let test_file = fs::read_to_string(test_path);
        let data: Data = serde_yaml::from_str(&test_file.unwrap()).unwrap();
        for x in data.launcher.arguments {
            java_command.arg(x);
        }
    }
    let id = java_command.current_dir(dir)
        .arg("-cp").arg(engine).
        arg("-jar").arg(game).spawn().unwrap().id();
    unsafe { discord_client(dir, id ) }
}

unsafe fn discord_client(dir: &str, id: u32) {
    //Ensure file was created
    let discord_file = Path::new(dir).join("discord.yml");

    let i = env!("DISCORD_KEY").parse().unwrap();
    let mut drpc = Client::new(i);
    drpc.start();
    println!("Starting Discord");
    while osspec::is_process_running(&id) {
        if discord_file.exists() {
            let test_file = fs::read_to_string(Path::new(dir).join("discord.yml"));
            let discord: Discord = serde_yaml::from_str(&test_file.unwrap()).unwrap();
            drpc.set_activity(|act| act.state(discord.current_task)).unwrap();
            thread::sleep(Duration::new(5, 0));
        }
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