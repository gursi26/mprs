// handles initial creation and parsing of config file
use crate::utils::{base_dir, config_path};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::{create_dir, create_dir_all, File, OpenOptions};
use std::path::PathBuf;

// Struct containing parsed config.yaml
#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigYAML {
    pub audio_format: Option<String>,
    pub display_visualizer: Option<bool>,
}

// Struct containing user config
#[derive(Debug)]
pub struct UserConfig {
    pub base_dir: PathBuf,
    pub audio_format: String,
    pub display_visualizer: bool,
}

impl Default for UserConfigYAML {
    fn default() -> Self {
        UserConfigYAML {
            audio_format: Some(String::from("mp3")),
            display_visualizer: Some(true),
        }
    }
}

// Initializes config if it doesnt exist. Does nothing if config file exists already.
pub fn init_config() {
    let config_path = config_path();

    let parent_path = config_path.parent().unwrap();
    if !parent_path.exists() {
        create_dir(parent_path).expect("Failed to create directory lmao");
    }
    if config_path.exists() {
        return;
    }
    // prompt user to ask for preferred music directory
    // ...
    // ~ -> home_dir
    // ./ -> executable curr_dir
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .expect("Could not open file.");

    let default_config = UserConfigYAML::default();
    // TODO change default_config.base_dir = preferred music_home_dir
    serde_yaml::to_writer(config_file, &default_config).unwrap();
}

// Fills potentially uninitialized values in config.yaml with defaults
fn fill_uninitialized_values(user_config_yaml: UserConfigYAML) -> UserConfig {
    let default_user_config_yaml = UserConfigYAML::default();

    let audio_format;
    let display_visualizer;

    match user_config_yaml.audio_format {
        Some(x) => audio_format = x,
        None => audio_format = default_user_config_yaml.audio_format.unwrap(),
    };
    match user_config_yaml.display_visualizer {
        Some(x) => display_visualizer = x,
        None => display_visualizer = default_user_config_yaml.display_visualizer.unwrap(),
    };

    UserConfig {
        base_dir: base_dir(),
        audio_format,
        display_visualizer,
    }
}

// Reads config.yaml and returns UserConfig struct
pub fn parse_config_file() -> UserConfig {
    let config_path = config_path();
    let config_file = File::open(config_path).expect("Could not find file lmao");
    let user_config_yaml =
        serde_yaml::from_reader(config_file).expect("Could not read values lmao");

    let filled_user_config = fill_uninitialized_values(user_config_yaml);
    let mut base_dir = filled_user_config.base_dir.clone();
    base_dir.push("liked");
    let _ = create_dir_all(base_dir);
    filled_user_config
}
