#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod args;
mod config;
mod ytdlp;

use args::*;
use clap::Parser;
use config::{init_config, parse_config_file, UserConfig};
use ytdlp::search_ytdlp;

fn main() {
    // init_config();
    // let user_config = parse_config_file();
    // let args = MusicPlayerArgs::parse();
    // match args.action_type {
    //     ActionType::Add(ref add_args) => mprs_add(add_args, &user_config),
    //     ActionType::Remove(ref remove_args) => mprs_remove(remove_args, &user_config),
    //     ActionType::Create(ref create_args) => mprs_create(create_args, &user_config),
    //     ActionType::Play(ref play_args) => mprs_play(play_args, &user_config),
    // };

    let (id_vec, results_vec) = search_ytdlp(String::from("ransom lil tecca"), 2);
    for x in results_vec {
        println!("{}, {}, {}", x.0, x.1, x.2);
    }
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
