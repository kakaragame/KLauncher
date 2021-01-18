extern crate reqwest;

use std::borrow::Borrow;
use std::fs;
use std::fs::{File, copy};
use std::path::Path;
use std::str::FromStr;

use self::reqwest::Url;
use std::io::Write;

const LATEST_BUILD: &str = "https://ci.potatocorp.dev/view/Kakara/job/Engine/lastSuccessfulBuild/artifact/";
const LATEST_JAR: &str = "https://ci.potatocorp.dev/view/Kakara/job/Engine/lastSuccessfulBuild/artifact/archives/";

//engine-1.0-9-SNAPSHOT-natives-windows.jar
pub async fn downloadLatestBuild() -> String {
    let resp = reqwest::get(Url::from_str(format!("{}{}", LATEST_BUILD, "files.txt").as_str()).unwrap()).await.unwrap().text().await.unwrap();

    let split = resp.split("\n");
    let vec: Vec<&str> = split.collect();
    let mut respond: String = String::new();
    for x in vec {
        if x.contains(getNativeName()) {
            let result = reqwest::get(Url::from_str(format!("{}{}", LATEST_JAR, x).as_str()).unwrap()).await.unwrap().bytes().await.unwrap();
            fs::create_dir_all("engine");
            let buf = Path::new("engine").join(x);
            if !buf.exists() {
                let mut file = File::create(buf).unwrap();
                file.write_all(result.as_ref());
            }

            respond = String::from(x);
            break;
        }
    }
    respond
}

fn getNativeName() -> &'static str {
    if cfg!(windows) {
        "native-windows"
    } else if cfg!(linux) {
        "native-linux"
    } else { "" }
}

