use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::ptr::null;
use std::str::FromStr;

use jenkins_api::JenkinsBuilder;
use jenkins_api::job::{CommonJob, Job, ShortJob};
use reqwest::Url;

pub fn getBranchURL(branch: &str) -> Result<CommonJob, String> {
    let jenkins = JenkinsBuilder::new("https://ci.potatocorp.dev/").build().unwrap();
    let job = jenkins.get_job("Kakara Engine 2").unwrap().as_variant::<jenkins_api::job::WorkflowMultiBranchProject>().unwrap();
    let vec = job.jobs;
    let mut value = Result::Err(format!("Unable to find branch: {}", branch));
    for x in vec {
        if x.name.eq(branch) {
            value = Result::Ok(x.get_full_job(&jenkins).unwrap());
        }
    }
    value
}

pub fn get_build_url(job: CommonJob) -> String {
    let jenkins = JenkinsBuilder::new("https://ci.potatocorp.dev/").build().unwrap();

    let build = job.last_build.unwrap();
    let build1 = build.get_full_build(&jenkins).unwrap();
    build1.url
}

pub async fn download_engine_jar(url: &str) -> String {
    let input = format!("{}artifact/{}", url, "files.txt");
    let resp = reqwest::get(Url::from_str(input.as_str()).unwrap()).await.unwrap().text().await.unwrap();
    let split = resp.lines();
    let vec: Vec<&str> = split.collect();
    let mut response: String = String::new();
    for x in vec {
        if x.contains(get_native_name()) {
            let string = format!("{}artifact/archives/{}", url, x);
            let result = reqwest::get(Url::from_str(string.as_str()).unwrap()).await.unwrap().bytes().await.unwrap();
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
//Kakara

pub async fn download_game(branch: &str) -> Result<String, String> {
    let jenkins = JenkinsBuilder::new("https://ci.potatocorp.dev/").build().unwrap();
    let job = jenkins.get_job("Kakara").unwrap().as_variant::<jenkins_api::job::WorkflowMultiBranchProject>().unwrap();
    let vec = job.jobs;
    let mut returnValue = Result::Err(format!("Unable to find branch: {}", branch));

    for x in vec {
        if x.name.eq(branch) {
            let build = x.get_full_job(&jenkins).unwrap().last_build.unwrap().get_full_build(&jenkins).unwrap();
            for x in build.artifacts {
                if (x.relative_path.starts_with("client/")) {

                    let value = (format!("{}artifact/{}", build.url, x.relative_path));
                    let buf1 = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("game");
                    if !buf1.exists() {
                        create_dir_all(buf1);
                    }
                    let string = x.file_name;
                    let buf = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("game").join(string);
                    returnValue = Result::Ok(String::from(buf.to_str().unwrap()));
                    let result = reqwest::get(Url::from_str(value.as_str()).unwrap()).await.unwrap().bytes().await.unwrap();
                    if !buf.exists() {
                        let mut file = File::create(buf).unwrap();
                        file.write_all(result.as_ref()).unwrap();
                    }
                    break;
                }
            }
        }
    }
    returnValue
}


fn get_native_name() -> &'static str {
    if cfg!(windows) {
        "natives-windows"
    } else if cfg!(linux) {
        "natives-linux"
    } else { "" }
}