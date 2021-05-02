use crate::utils;
use std::fs;
use serde::{Serialize, Deserialize, Serializer};
use crate::error::KError;
use crate::error::Level;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct GameProfile {
    pub name: String,
    pub jre: String,
    //Format source:version ex release:1.0.0, jenkins:master, file:"path_to_file"
    pub version: String,
    //Format source:version ex release:1.0.0, jenkins:master, file:"path_to_file", or default
    pub engine_version: String,
    pub game_directory: String,
    pub jvm_args: Vec<String>,
    pub test_mode: bool,

}


impl GameProfile {
    pub fn set_jre(&mut self, jre: String) {
        self.jre = jre;
    }
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    pub fn set_engine_version(&mut self, version: String) {
        self.engine_version = version;
    }

    pub fn set_jvm(&mut self, jvm: String) {
        self.jre = jvm;
    }
    pub fn set_game_directory(&mut self, dir: String) {
        self.game_directory = dir;
    }
    pub fn set_test_mode(&mut self, test_mode: bool) {
        self.test_mode = test_mode;
    }
    pub fn set_jvm_args(&mut self, args: Vec<String>) {
        self.jvm_args = args;
    }


    pub fn new(name: String, version: String, engine_version: String, test_mode: bool) -> GameProfile {
        GameProfile {
            name,
            version,
            engine_version,
            jre: String::new(),
            game_directory: String::new(),
            jvm_args: Vec::new(),
            test_mode,
        }
    }
    pub fn update_profile(profile: GameProfile) -> Result<(), KError> {
        let result = GameProfile::get_profiles();

        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let mut profiles = result.ok().unwrap();

        for mut x in profiles{
            if x.name.eq(&profile.name) {
                x = profile.clone();
            }
        }
        return Ok(());
    }
    pub fn del_profile(profile: GameProfile) -> Result<(), KError> {
        let result = GameProfile::get_profiles();

        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let mut profiles = result.ok().unwrap();

        for (pos, e) in profiles.iter().enumerate() {
            if e.name.eq(&profile.name) {
                profiles.remove(pos);
                break;
            }
        }
        return Ok(());
    }
    pub fn add_profile(profile: GameProfile) -> Result<(), KError> {
        let result = GameProfile::get_profiles();

        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let mut profiles = result.ok().unwrap();


        profiles.push(profile);
        GameProfile::save_profiles(profiles);
        return Ok(());
    }
    fn save_profiles(profiles: Vec<GameProfile>) -> Result<(), KError> {
        let result = serde_json::to_string(&profiles).unwrap();
        let file = utils::get_kakara_folder().join("profiles.json");
        if file.exists() {
            let result1 = fs::remove_file(&file);
            if result1.is_err() {
                return Err(KError::new_medium("Unable to delete file"));
            }
        }
        let result2 = fs::write(&file, &result);
        if result2.is_err() {
            return Err(KError::new_medium("Unable to save file"));
        }
        Ok(())
    }
    pub fn get_profiles() -> Result<Vec<GameProfile>, KError> {
        let file = utils::get_kakara_folder().join("profiles.json");
        let file_content = fs::read_to_string(&file);
        if file_content.is_err() {
            return Err(KError::new_severe("Unable to read file"));
        }
        let profiles: Vec<GameProfile> = serde_json::from_str(&file_content.unwrap()).unwrap();
        return Ok(profiles);
    }
}