use std::path::Path;

use std::fs::create_dir_all;
use std::fs;
use crate::kconfig::TestConfig;
use crate::kconfig::Auth;
use crate::kconfig::Launcher;
use crate::kconfig::Settings;
pub fn is_installed(path: &Path) -> bool {
    path.clone().join("test").join("test.yml").exists()
}

pub fn install(working_directory: &Path) {
    let launcher = Launcher::new();
    let auth = Auth::new();
    let config = TestConfig {
        launcher,
        auth,
    };
    let mut buf = working_directory.clone().join("test");
    if !buf.exists(){
        create_dir_all(&buf).unwrap();
    }
    buf = buf.join("test.yml");
    let result = serde_yaml::to_string(&config).unwrap();
    fs::write(&buf, &result).expect("Unable to write file");

}