#[derive(Serialize, Deserialize)]
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
    pub fn set_name(&mut self, name: String) {
        self.name = name;
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
            name: name,
            version: version,
            engine_version: engine_version,
            jre: String::new(),
            game_directory: String::new(),
            jvm_args: Vec::new(),
            test_mode: test_mode,
        }
    }
    pub fn update_profile(profile: GameProfile) {
        //TODO implement
    }
    pub fn del_profile(profile: GameProfile) {
        //TODO implement
    }
    pub fn add_profile(profile: GameProfile) {
        //TODO implement
    }
    pub fn get_profiles() -> Vec<GameProfile> {
        //TODO implement
        Vec::new()
    }
}