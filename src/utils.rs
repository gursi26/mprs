use std::{path::PathBuf, fs::{File, OpenOptions}};
use dirs::home_dir;

pub fn config_path() -> PathBuf {
    let mut config_path = home_dir().unwrap();
    config_path.push(".config");
    config_path.push("mprs");
    config_path.push("config.yaml");
    config_path
}

pub fn base_dir() -> PathBuf {
    let mut base_dir = home_dir().unwrap();
    base_dir.push("mprs-music");
    base_dir
}
