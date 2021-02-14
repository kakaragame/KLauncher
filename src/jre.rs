//https://api.adoptopenjdk.net/v3/binary/latest/11/ga/{}/x64/jre/hotspot/normal/adoptopenjdk?project=jdk

use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};

use crate::{utils, downloader};

fn get_jre_version() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(linux) {
        "linux"
    } else { "" }
}

fn get_file_extension() -> &'static str {
    if cfg!(windows) {
        "zip"
    } else if cfg!(linux) {
        "tar.gz"
    } else { "" }
}

fn get_java_exec() -> &'static str {
    if cfg!(windows) {
        "java.exe"
    } else if cfg!(linux) {
        "java"
    } else { "" }
}

pub async fn download_jre() -> PathBuf {
    let url = format!("https://api.adoptopenjdk.net/v3/binary/latest/11/ga/{}/x64/jre/hotspot/normal/adoptopenjdk?project=jdk", get_jre_version());
    let folder = utils::get_kakara_folder().join("jre");
    let downloads = utils::get_kakara_folder().join("downloads");
    if folder.exists() {
        remove_dir_all(&folder);
    }    if downloads.exists() {
        remove_dir_all(&downloads);
    }

    create_dir_all(&downloads);
    create_dir_all(&folder);

    let jre_download = downloads.join(format!("download.{}", get_file_extension()));
    downloader::download(&url, &jre_download, &"Jre 11").await;
    extract(&jre_download, &folder);
    folder.join("bin").join(get_java_exec())
}

#[cfg(windows)]
pub fn extract(file: &Path, extractTo: &Path) {}

#[cfg(unix)]
pub fn extract(file: &Path, extractTo: &Path) {}
