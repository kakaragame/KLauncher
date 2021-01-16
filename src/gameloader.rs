extern crate yaml_rust;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

use yaml_rust::{YamlEmitter, YamlLoader};

pub fn load(game: &str, dir: &str, engine: &str, debug: bool) {
    let home = std::env::var("HOME").unwrap();
    let path = Path::new(&home).join(".kakara").join("settings.yml");
    let ymlString = fs::read_to_string(path);
    let docs = YamlLoader::load_from_str(&*ymlString.unwrap()).unwrap();
    let program = docs[0]["java"].as_str().unwrap();
    println!("Using java {}", program);
    let mut java_command = Command::new(program)
        .current_dir(dir)
        .arg("-cp").arg(engine).
        arg("-jar").arg(game).
        spawn();
}
