use crate::error::LauncherError::HTTPError;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("HTTP Error Code '{0}'")]
    HTTPError(StatusCode),
    #[error("IO error {0}")]
    IOError(#[from]std::io::Error),
    #[error("Reqwest had an Error {0}")]
    ReqwestError(#[from]reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    YamlError(#[from]serde_yaml::Error),
    #[error("Internal Error {0}")]
    Custom(String),
}


impl From<StatusCode> for LauncherError {
    fn from(err: StatusCode) -> LauncherError {
        HTTPError(err)
    }
}
