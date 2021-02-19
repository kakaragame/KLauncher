use std::path::Path;
use shrust::{Shell, ShellIO};
use crate::settings;
use crate::settings::Auth;
use crate::settings::Launcher;
use crate::settings::TestConfig;
pub fn is_installed(path: &Path) -> bool {
    path.clone().join("test").join("test.yml").exists()
}

pub fn install() {
    let v = Vec::new();
    let launcher = Launcher::new();
    let mut shell = Shell::new(v);
    shell.new_command("jre", "Overrides the Current JRE", 1, |io, v, s| {
        println!("Pushing {}", s[0]);
        v.push(s[0].to_string());
        Ok(())
    });


    shell.run_loop(&mut ShellIO::default());
}