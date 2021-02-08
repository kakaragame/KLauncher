use jenkins_api::JenkinsBuilder;
use jenkins_api::job::{Job, ShortJob, CommonJob};
use std::ptr::null;
use std::error::Error;
use reqwest::Url;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub fn getBranchURL(branch: &str) -> Result<CommonJob, String> {
    let jenkins = JenkinsBuilder::new(env!("JENKINS_URL")).build().unwrap();
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
    let jenkins = JenkinsBuilder::new(env!("JENKINS_URL")).build().unwrap();

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


fn get_native_name() -> &'static str {
    if cfg!(windows) {
        "natives-windows"
    } else if cfg!(linux) {
        "natives-linux"
    } else { "" }
}