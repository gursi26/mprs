// handles initial creation and parsing of config file
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::{create_dir, create_dir_all, File, OpenOptions};
use std::path::PathBuf;

// Struct containing parsed config.yaml
#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigYAML {
    pub base_dir: Option<PathBuf>,
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
        let mut home_dir = home_dir().unwrap();
        home_dir.push("mprs");

        UserConfigYAML {
            base_dir: Some(home_dir),
            audio_format: Some(String::from("mp3")),
            display_visualizer: Some(true),
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        let mut home_dir = home_dir().unwrap();
        home_dir.push("mprs");

        UserConfig {
            base_dir: home_dir,
            audio_format: String::from("mp3"),
            display_visualizer: true,
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
    let mut user_config = UserConfig::default();

    if let Some(base_dir) = user_config_yaml.base_dir {
        user_config.base_dir = base_dir
    }

    if let Some(audio_format) = user_config_yaml.audio_format {
        user_config.audio_format = audio_format
    }

    if let Some(display_visualizer) = user_config_yaml.display_visualizer {
        user_config.display_visualizer = display_visualizer
    }

    user_config
}

fn create_base_dir(user_config: &UserConfig) {
    let mut cloned_base_dir = user_config.base_dir.clone();
    cloned_base_dir.push("liked");
    create_dir_all(cloned_base_dir).unwrap();
}

// Reads config.yaml and returns UserConfig struct
pub fn parse_config_file() -> UserConfig {
    let mut config_path = home_dir().unwrap();
    config_path.push(".config");
    config_path.push("mprs");
    config_path.push("config.yaml");

    let config_file = File::open(config_path).expect("Could not find file lmao");
    let user_config_yaml =
        serde_yaml::from_reader(config_file).expect("Could not read values lmao");
    let filled_user_config = fill_uninitialized_values(user_config_yaml);
    create_base_dir(&filled_user_config);
    filled_user_config
}
