use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::{fs, thread};

use serde::Deserialize;

use crate::settings;

use crate::{osspec, LauncherError};

pub fn load(game: PathBuf, dir: PathBuf, engine: PathBuf) -> Result<(), LauncherError> {
    let working = dir;

    //working = fs::canonicalize(working)?;
    println!(
        "[DEBUG] working directory: {}",
        working.as_os_str().to_str().unwrap()
    );
    println!("[DEBUG] engine: {}", engine.as_os_str().to_str().unwrap());
    println!("[DEBUG] game: {}", game.as_os_str().to_str().unwrap());

    if !working.exists() {
        create_dir_all(working.as_path());
    }
    if !engine.exists() {
        panic!(
            "Engine Jar not found in {}",
            engine.as_os_str().to_str().unwrap()
        )
    }
    if !game.exists() {
        panic!(
            "Game Jar not found in {}",
            game.as_os_str().to_str().unwrap()
        )
    }
    let mut home;
    // .kakara location is different depending on the OS.
    if cfg!(windows) {
        // Use %appdata% if in windows.
        home = Path::new(&format!(
            "{}{}{}",
            std::env::var("USERPROFILE").unwrap(),
            "\\AppData",
            "\\Roaming"
        ))
        .join(".kakara");
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
    if !home.exists() {
        panic!("User config not found in {}", home.to_str().unwrap())
    }
    println!("Kakara Config {:?}", home.to_str());
    let yml_string = fs::read_to_string(home);
    let settings: settings::Settings = serde_yaml::from_str(&yml_string.unwrap()).unwrap();
    let test_path = working.join("test.yml");

    let test_file = fs::read_to_string(&test_path);
    let mut java = settings.java;
    let data: settings::TestConfig = serde_yaml::from_str(&test_file.unwrap()).unwrap();
    if !data.launcher.jre.is_empty() {
        java = data.launcher.jre;
    }
    println!("[DEBUG] {}", &java);

    let mut java_command = Command::new(java);
    if test_path.exists() {
        println!("[DEBUG] Using custom arguments");
        for x in data.launcher.arguments {
            if !x.is_empty() {
                java_command.arg(x);
            }
        }
    }

    let id = java_command
        .current_dir(working)
        .arg("-jar")
        .arg(&game)
        .arg(format!(
            "{}={}",
            "--engine",
            engine.as_os_str().to_str().unwrap()
        ))
        .spawn()
        .unwrap()
        .id();

    #[cfg(feature = "discord")]
    {
        discord_client(&dir, id);
    }
    #[cfg(not(feature = "discord"))]
    {
        println!("Discord integration is disabled.");
        while osspec::is_process_running(id) {
            thread::sleep(Duration::new(5, 0));
        }
    }
    Ok(())
}

#[cfg(feature = "discord")]
fn discord_client(dir: &PathBuf, id: u32) {
    //Ensure file was created
    let discord_file = dir.join("discord.yml");

    let i = option_env!("DISCORD_KEY");
    if let Some(value) = i {
        let mut drpc = discord_rpc_client::Client::new(value.parse().unwrap());
        drpc.start();
        println!("Starting Discord");
        while osspec::is_process_running(id) {
            if discord_file.exists() {
                let test_file = fs::read_to_string(Path::new(dir).join("discord.yml"));
                let discord: Discord = serde_yaml::from_str(&test_file.unwrap()).unwrap();
                drpc.set_activity(|act| act.state(discord.current_task))
                    .unwrap();
                thread::sleep(Duration::new(5, 0));
            }
            thread::sleep(Duration::new(5, 0));
        }
    }
}
#[cfg(feature = "discord")]
#[derive(Deserialize)]
struct Discord {
    current_task: String,
}
