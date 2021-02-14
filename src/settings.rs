use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize,Deserialize)]
pub(crate) struct Settings {
    pub java: String
}