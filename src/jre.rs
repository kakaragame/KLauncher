//https://api.adoptopenjdk.net/v3/binary/latest/11/ga/{}/x64/jre/hotspot/normal/adoptopenjdk?project=jdk

use std::fs::{create_dir_all, File, remove_dir_all};
use std::{fs, thread, time};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{downloader, utils};

fn get_jre_version() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(unix) {
        "linux"
    } else { "" }
}

fn get_file_extension() -> &'static str {
    if cfg!(windows) {
        "zip"
    } else if cfg!(unix) {
        "tar.gz"
    } else { "" }
}

fn get_java_exec() -> &'static str {
    if cfg!(windows) {
        "java.exe"
    } else if cfg!(unix) {
        "java"
    } else { "" }
}

pub async fn download_jre() -> PathBuf {
    let url = format!("https://api.adoptium.net/v3/binary/latest/17/ga/{}/x64/jre/hotspot/normal/eclipse?project=jdk", get_jre_version());
    println!("{}", &url);
    let mut folder = utils::get_kakara_folder().join("jre");
    let downloads = utils::get_kakara_folder().join("downloads");
    if folder.exists() {
        remove_dir_all(&folder).unwrap();
    }
    if downloads.exists() {
        remove_dir_all(&downloads).unwrap();
    }

    create_dir_all(&downloads).unwrap();
    create_dir_all(&folder).unwrap();

    let jre_download = downloads.join(format!("download.{}", get_file_extension()));
    let result = downloader::download(&url, &jre_download, &"JRE 17").await;
    if result.is_err(){
        println!("Unable to download {}", result.err().unwrap());
        panic!("Unable to download JRE");
    }
    extract(&jre_download, &folder);

    let file1 = fs::read_dir(&folder).unwrap();
    for x in file1 {
        folder = folder.join(x.unwrap().file_name().to_str().unwrap());
        break;
    }

    folder.join("bin").join(get_java_exec())
}

#[cfg(windows)]
pub fn extract(file: &Path, extract_to: &Path) {
    let mut file = File::open(&file);
    let mut archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    archive.extract(extract_to).unwrap();
}

#[cfg(unix)]
pub fn extract(file: &Path, extractTo: &Path) {
    //tar -xzvf {file} -C {extractTo}
     Command::new("tar").arg("-xzf").arg(file.to_str().unwrap()).arg("-C").arg(extractTo.to_str().unwrap()).spawn().unwrap().wait().unwrap();
}
