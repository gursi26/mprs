#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod args;
mod config;

use clap::Parser;
use args::*;
use config::{init_config, parse_config_file, UserConfig};

fn main() {
    init_config();
    let user_config = parse_config_file();
    let args = MusicPlayerArgs::parse();
    match args.action_type {
        ActionType::Add(ref add_args) => mprs_add(add_args, &user_config),
        ActionType::Remove(ref remove_args) => mprs_remove(remove_args, &user_config),
        ActionType::Create(ref create_args) => mprs_create(create_args, &user_config),
        ActionType::Play(ref play_args) => mprs_play(play_args, &user_config),
    };
}

fn mprs_add(args: &AddArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}

fn mprs_remove(args: &RemoveArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}

fn mprs_create(args: &CreateArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}

fn mprs_play(args: &PlayArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}
