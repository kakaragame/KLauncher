use std::path::{Path, PathBuf};

pub fn get_kakara_folder() -> PathBuf {
    // .kakara location is different depending on the OS.
    if cfg!(windows) {
        // Use %appdata% if in windows.
        return Path::new(&format!(
            "{}{}{}",
            std::env::var("USERPROFILE").unwrap(),
            "\\AppData",
            "\\Roaming"
        ))
        .join(".kakara");
    } else if cfg!(linux) {
        // Use /home/{user} if on linux. (This may work for mac. Currently untested.)
        return Path::new(&std::env::var("HOME").unwrap()).join(".kakara");
    } else if cfg!(osx) {
        return Path::new(&std::env::var("HOME").unwrap()).join("kakara");
    } else {
        // Else attempt to find the home environment variable.
        return Path::new(&std::env::var("HOME").unwrap()).join(".kakara");
    }
}
