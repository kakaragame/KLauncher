use std::fs;
use std::fs::create_dir_all;

use crate::jre::download_jre;
use crate::settings::Settings;
use crate::utils;

pub fn is_installed() -> bool {
    utils::get_kakara_folder().join("settings.yml").exists()
}

pub async fn install() {
    let buf = utils::get_kakara_folder();
    if !buf.exists() {
        create_dir_all(buf).unwrap();
    }
    let mut jre = download_jre().await;
    let java_location = jre.to_str().unwrap();
    let settings = Settings {
        java: java_location.parse().unwrap(),
    };
    let result = serde_yaml::to_string(&settings).unwrap();
    fs::write(
        utils::get_kakara_folder()
            .join("settings.yml")
            .to_str()
            .unwrap(),
        &result,
    )
    .expect("Unable to write file");
}
