use crate::settings::Auth;
use crate::settings::Launcher;
use crate::settings::TestConfig;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

pub fn is_installed(path: impl AsRef<Path>) -> bool {
    path.as_ref().join("test").join("test.yml").exists()
}

pub fn install(working_directory: impl AsRef<Path>) {
    let launcher = Launcher::new();
    let auth = Auth::new();
    let config = TestConfig { launcher, auth };
    let mut buf = working_directory.as_ref().join("test");
    if !buf.exists() {
        create_dir_all(&buf).unwrap();
    }
    buf = buf.join("test.yml");
    let result = serde_yaml::to_string(&config).unwrap();
    fs::write(&buf, &result).expect("Unable to write file");
}
