mod args;
mod config;

use clap::Parser;
use args::*;
use config::*;

fn main() {
    let args = MusicPlayerArgs::parse();
    match args.action_type {
        ActionType::Add(ref add_args) => mprs_add(add_args),
        ActionType::Remove(ref remove_args) => mprs_remove(remove_args),
        ActionType::Create(ref create_args) => mprs_create(create_args),
        ActionType::Play(ref play_args) => mprs_play(play_args),
    };
}

fn mprs_add(args: &AddArgs) {
    println!("{:?}", args);
}

fn mprs_remove(args: &RemoveArgs) {
    println!("{:?}", args);
}

fn mprs_create(args: &CreateArgs) {
    println!("{:?}", args);
}

fn mprs_play(args: &PlayArgs) {
    println!("{:?}", args);
}
