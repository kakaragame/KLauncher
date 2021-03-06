use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::ptr::null;
use std::str::FromStr;

use jenkins_api::JenkinsBuilder;
use jenkins_api::job::{CommonJob, Job, ShortJob};
use crate::downloader;
use reqwest::Url;

pub fn get_branch_url(branch: &str) -> Result<CommonJob, String> {
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

/**
   Download the engine jar.

   # Params
   url -> The url for the engine's jenkins (without the /artifact)

   # Returns
   String -> The name of the engine that was downloaded.

*/
pub async fn download_engine_jar(url: &str) -> String {


    let input = format!("{}artifact/{}", url, "files.txt");
    let resp = reqwest::get(Url::from_str(input.as_str()).unwrap()).await.unwrap().text().await.unwrap();
    let split = resp.lines();
    let vec: Vec<&str> = split.collect();
    let mut response: String = String::new();
    for x in vec {
        if x.contains(get_native_name()) {
            let string = format!("{}artifact/archives/{}", url, x);
            fs::create_dir_all("engine").unwrap();
            let buf1 = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine");
            if !buf1.exists() {
                create_dir_all(buf1);
            }
            let buf = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine").join(x);
            downloader::download(&string.as_str(), &buf.as_path(), "Kakara Engine").await.unwrap();


            response = String::from(x);
            break;
        }
    }
    response
}
//Kakara

/**
   Download the kakara game jar.

   # Params
   branch -> The branch that should be downloaded.

   # Returns
   Result<String, String> -> The result of the download.

*/
pub async fn download_game(branch: &str) -> Result<String, String> {
    let jenkins = JenkinsBuilder::new("https://ci.potatocorp.dev/").build().unwrap();
    let job = jenkins.get_job("Kakara").unwrap().as_variant::<jenkins_api::job::WorkflowMultiBranchProject>().unwrap();
    let vec = job.jobs;
    let mut return_value = Result::Err(format!("Unable to find branch: {}", branch));

    for x in vec {
        if x.name.eq(branch) {
            let build = x.get_full_job(&jenkins).unwrap().last_build.unwrap().get_full_build(&jenkins).unwrap();
            for x in build.artifacts {
                if (x.relative_path.starts_with("client/")) {
                    let value = (format!("{}artifact/{}", build.url, x.relative_path));
                    let string = x.file_name;
                    let buf = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("game").join(string);
                    downloader::download(&value, &buf.as_path(), "Kakara game").await.unwrap();
                    return_value = Result::Ok(String::from(buf.to_str().unwrap()));
                    break;
                }
            }
        }
    }
    return_value
}


/**
   Get the native name of the operating system.
   (Only supports linux and windows)

   # Returns
   str -> The native name of the operating system.

*/
fn get_native_name() -> &'static str {
    if cfg!(windows) {
        "natives-windows"
    } else if cfg!(linux) {
        "natives-linux"
    } else { "" }
}