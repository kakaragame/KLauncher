use std::env::current_dir;
use std::fs;
use std::fs::create_dir_all;

use std::path::Path;

use std::str::FromStr;

use crate::downloader;
use crate::error::LauncherError;
use jenkins_api::job::Job;
use jenkins_api::JenkinsBuilder;
use reqwest::Url;
const JENKINS_URL: &str = "https://ci.kingtux.dev";
/**
Download the engine jar.

# Params
url -> The url for the engine's jenkins (without the /artifact)

# Returns
String -> The name of the engine that was downloaded.

 */
pub fn download_engine_jar(branch: &str) -> Result<String, LauncherError> {
    let input = format!("{JENKINS_URL}/view/Kakara/job/Kakara%20Engine%202/job/{branch}/lastSuccessfulBuild/artifact/files.txt" );
    let resp = reqwest::blocking::get(Url::from_str(input.as_str()).unwrap())
        .unwrap()
        .text()
        .unwrap();
    let split = resp.lines();
    let vec: Vec<&str> = split.collect();
    for x in vec {
        if x.contains(get_native_name()) {
            let string = format!("{JENKINS_URL}/view/Kakara/job/Kakara%20Engine%202/job/{}/lastSuccessfulBuild/artifact/archives/{}",&branch ,x);
            fs::create_dir_all("engine").unwrap();
            let buf1 = Path::new(std::env::current_exe().unwrap().parent().unwrap()).join("engine");
            if !buf1.exists() {
                create_dir_all(buf1);
            }
            let buf = current_dir().unwrap().join("engine").join(x);
            downloader::download(string.as_str(), buf.as_path(), "Kakara Engine")?;

            return Ok(x.to_string());
        }
    }
    Err(LauncherError::Custom(
        "Unable to download engine".to_string(),
    ))
}
//Kakara

/**
Download the kakara game jar.

# Params
branch -> The branch that should be downloaded.

# Returns
Result<String, String> -> The result of the download.

 */
pub fn download_game(branch: &str) -> Result<String, String> {
    let jenkins = JenkinsBuilder::new(JENKINS_URL).build().unwrap();
    let kakara_job = jenkins
        .get_job("Kakara")
        .unwrap()
        .as_variant::<jenkins_api::job::WorkflowMultiBranchProject>()
        .unwrap();
    let vec = kakara_job.jobs;
    let mut return_value = Result::Err(format!("Unable to find branch: {}", branch));

    for sub_job in vec {
        if sub_job.name.eq(branch) {
            let build = sub_job
                .get_full_job(&jenkins)
                .unwrap()
                .last_build
                .unwrap()
                .get_full_build(&jenkins)
                .unwrap();
            for x in build.artifacts {
                if x.relative_path.starts_with("client/") {
                    let value = format!("{}artifact/{}", build.url, x.relative_path);
                    let string = x.file_name;
                    let buf = current_dir().unwrap().join("game").join(string);
                    downloader::download(&value, buf.as_path(), "Kakara game").unwrap();
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
    } else {
        ""
    }
}
