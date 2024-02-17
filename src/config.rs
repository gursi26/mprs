// handles initial creation and parsing of config file
use std::fs::{create_dir, File, OpenOptions};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use dirs::home_dir;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub base_dir: Option<PathBuf>,
    pub audio_format: Option<String>,
    pub display_visualizer: Option<bool>,
}

impl Default for UserConfig {
    fn default() -> Self {
        let mut home_dir = home_dir().unwrap();
        home_dir.push("mprs");

        UserConfig {
            base_dir: Some(home_dir),
            audio_format: Some(String::from("mp3")),
            display_visualizer: Some(true),
        }
    }
}

// Initializes config if it doesnt exist. Does nothing if config file exists already.
pub fn init_config() {
    let mut config_path = home_dir().unwrap();
    config_path.push(".config");
    config_path.push("mprs");
    config_path.push("config.yaml");

    let parent_path = config_path.parent().unwrap();
    if !parent_path.exists() {
        create_dir(parent_path).expect("Failed to create directory lmao");
    }
    if config_path.exists() {
        return;
    }
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .expect("Could not open file.");

    let default_config = UserConfig::default();
    serde_yaml::to_writer(config_file, &default_config).unwrap();
}

fn fill_uninitialized_values(user_config: &mut UserConfig) {
    let default_user_config = UserConfig::default();

    match user_config.base_dir {
        None => user_config.base_dir = default_user_config.base_dir,
        _ => ()
    }

    match user_config.audio_format {
        None => user_config.audio_format = default_user_config.audio_format,
        _ => ()
    }

    match user_config.display_visualizer {
        None => user_config.display_visualizer = default_user_config.display_visualizer,
        _ => ()
    }
}

pub fn parse_config_file() -> UserConfig {
    let mut config_path = home_dir().unwrap();
    config_path.push(".config");
    config_path.push("mprs");
    config_path.push("config.yaml");

    let config_file = File::open(config_path).expect("Could not find file lmao");
    let mut user_config = serde_yaml::from_reader(config_file).expect("Could not read values lmao");
    fill_uninitialized_values(&mut user_config);
    user_config
}
