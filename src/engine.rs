extern crate reqwest;

use std::fs;
use std::fs::{File};
use std::path::Path;
use std::str::FromStr;

use self::reqwest::Url;
use std::io::Write;

const LATEST_BUILD: &str = "https://ci.potatocorp.dev/view/Kakara/job/Engine/lastSuccessfulBuild/artifact/";
const LATEST_JAR: &str = "https://ci.potatocorp.dev/view/Kakara/job/Engine/lastSuccessfulBuild/artifact/archives/";

//engine-1.0-9-SNAPSHOT-natives-windows.jar
pub async fn download_latest_build(workingDir: &str) -> String {
    let resp = reqwest::get(Url::from_str(format!("{}{}", LATEST_BUILD, "files.txt").as_str()).unwrap()).await.unwrap().text().await.unwrap();

    let split = resp.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut response: String = String::new();
    for x in vec {
        if x.contains(get_native_name()) {
            let result = reqwest::get(Url::from_str(format!("{}{}", LATEST_JAR, x).as_str()).unwrap()).await.unwrap().bytes().await.unwrap();
            fs::create_dir_all("engine").unwrap();
            let buf = Path::new(workingDir).join("engine").join(x);
            if !buf.exists() {
                let mut file = File::create(buf).unwrap();
                file.write_all(result.as_ref()).unwrap();
            }

            response = String::from(x);
            break;
        }
    }
    response
}

fn get_native_name() -> &'static str {
    if cfg!(windows) {
        "native-windows"
    } else if cfg!(linux) {
        "native-linux"
    } else { "" }
}

