use jenkins_api::JenkinsBuilder;
use jenkins_api::job::{Job, ShortJob, CommonJob};
use std::ptr::null;
use std::error::Error;

pub fn getBranchURL(branch: &str) -> Result<ShortJob<CommonJob>, &str> {
    let jenkins = JenkinsBuilder::new(env!("JENKINS_URL")).build().unwrap();
    let job = jenkins.get_job("Kakara Engine 2").unwrap().as_variant::<jenkins_api::job::WorkflowMultiBranchProject>().unwrap();
    let vec = job.jobs;
    let mut value = Result::Err("Missing Data");
    for x in vec {
        if x.name.eq(branch) {
            value = Result::Ok(x);
        }
    }
    value
}

fn get_native_name() -> &'static str {
    if cfg!(windows) {
        "natives-windows"
    } else if cfg!(linux) {
        "natives-linux"
    } else { "" }
}