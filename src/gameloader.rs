use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::ops::Index;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

pub fn load(game: &str, dir: &str, engine: &str, debug: bool) {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".kakara").join("settings.yml");
    let ymlString = fs::read_to_string(path);
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