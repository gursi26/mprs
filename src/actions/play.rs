use crate::args::PlayArgs;
use crate::config::UserConfig;

pub fn mprs_play(args: &PlayArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}
