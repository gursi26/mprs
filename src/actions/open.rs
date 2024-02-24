use crate::config::UserConfig;
use std::process::{Command, Stdio};
use std::env;

pub fn mprs_open(config: &UserConfig) {
    let command = match env::consts::OS {
        "macos" => "open",
        "linux" => "xdg-open",
        "windows" => "explorer",
        &_ => "",
    };
    let output = Command::new(command)
        .arg(config.base_dir.to_str().unwrap())
        .spawn()
        .unwrap();
}
