use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

use java_properties::read;

pub fn load(game: &str, dir: &str, engine: &str, debug: bool) {
    let home = std::env::var("HOME").unwrap();
    let mut f2 = File::open(Path::new(&home).join(".kakara").join("settings.properties"));
    let dst_map1 = read(BufReader::new(f2.unwrap()));
    let map = dst_map1.unwrap();
    let option = map.get("java").unwrap();
    let mut java_command = Command::new(option.as_str())
        .current_dir(dir)
        .arg("-cp").arg(engine).
        arg("-jar").arg(game).
        spawn();
}
