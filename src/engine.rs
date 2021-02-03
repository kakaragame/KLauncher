extern crate reqwest;

use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use self::reqwest::Url;

const LATEST_BUILD: &str = "https://ci.kingtux.dev/view/Kakara/job/Kakara%20Engine%202/job/master/lastSuccessfulBuild/artifact/";
const LATEST_JAR: &str = "https://ci.kingtux.dev/view/Kakara/job/Kakara%20Engine%202/job/master/lastSuccessfulBuild/artifact/archives/";

//engine-1.0-9-SNAPSHOT-natives-windows.jar
/**
    Downloads the latest engine build for the OS and returns the name.

   # Params
    working_dir -> Unused

   # Returns
    The name of the latest build

   # Examples
   ```rust
    let name: String = engine::download_latest_build("unused");
   ```
*/
pub async fn download_latest_build(working_dir: &str) -> String {
    let resp = reqwest::get(Url::from_str(format!("{}{}", LATEST_BUILD, "files.txt").as_str()).unwrap()).await.unwrap().text().await.unwrap();
    let split = resp.lines();
    let vec: Vec<&str> = split.collect();
    let mut response: String = String::new();
    for x in vec {
        if x.contains(get_native_name()) {
            let result = reqwest::get(Url::from_str(format!("{}{}", LATEST_JAR, x).as_str()).unwrap()).await.unwrap().bytes().await.unwrap();
            fs::create_dir_all("engine").unwrap();
            let buf1 = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine");
            if !buf1.exists() {
                create_dir_all(buf1);
            }
            let buf = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine").join(x);
            println!("[DEBUG] Downloaded: {}", buf.as_path().as_os_str().to_str().unwrap());
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
        "natives-windows"
    } else if cfg!(linux) {
        "natives-linux"
    } else { "" }
}

