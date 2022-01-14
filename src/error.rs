use reqwest::StatusCode;
use thiserror::Error;
use crate::error::LauncherError::HTTPError;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("HTTP Error Code '{0}'")]
    HTTPError(StatusCode),
    #[error("IO error {0}")]
    IOError(std::io::Error),
    #[error("Reqwest had an Error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    YamlError(serde_yaml::Error),
    #[error("Internal Error {0}")]
    Custom(String),
}

impl From<reqwest::Error> for LauncherError {
    fn from(err: reqwest::Error) -> LauncherError {
        LauncherError::ReqwestError(err)
    }
}

impl From<StatusCode> for LauncherError {
    fn from(err: StatusCode) -> LauncherError {
        match err {
            value => {
                return HTTPError(value);
            }
        }
    }
}

impl From<serde_yaml::Error> for LauncherError {
    fn from(err: serde_yaml::Error) -> LauncherError {
        LauncherError::YamlError(err)
    }
}impl From<std::io::Error> for LauncherError {
    fn from(err: std::io::Error) -> LauncherError {
        LauncherError::IOError(err)
    }
}