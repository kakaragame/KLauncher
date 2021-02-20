use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub(crate) struct Settings {
    pub java: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TestConfig {
    pub launcher: Launcher,
    pub auth: Auth,

}

#[derive(Serialize, Deserialize)]
pub(crate) struct Launcher {
    pub arguments: Vec<String>,
    pub jre: String,

}

impl Launcher {
    pub fn set_jre(&mut self, jre: String) {
        self.jre = jre;
    }

    pub fn set_arguments(&mut self, args: Vec<String>) {
        self.arguments = args;
    }
    pub fn new() -> Launcher {
        Launcher {
            arguments: Vec::new(),
            jre: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Auth {
    pub auth_package: String,
    pub uuid: String,
    pub auth_key: String,

}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            auth_package: String::from(""),
            uuid: String::from(""),
            auth_key: String::from(""),
        }
    }
}
