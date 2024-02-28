#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod args;
mod config;
mod utils;
mod ytdlp;
mod actions;
use crate::actions::add::mprs_add;
use crate::actions::create::mprs_create;
use crate::actions::remove::mprs_remove;
use crate::actions::play::mprs_play;
use crate::actions::list::mprs_list;
use crate::actions::open::mprs_open;
use crate::actions::mv_cpy::mprs_move;
use crate::ytdlp::ytdlp_get_info_from_link;

use args::*;
use clap::Parser;
use config::{init_config, parse_config_file};

fn main() {
    init_config();
    let user_config = parse_config_file();
    let args = MusicPlayerArgs::parse();
    match args.action_type {
        ActionType::Add(ref add_args) => mprs_add(add_args, &user_config),
        ActionType::Remove(ref remove_args) => mprs_remove(remove_args, &user_config),
        ActionType::Create(ref create_args) => mprs_create(create_args, &user_config),
        ActionType::Play(ref play_args) => mprs_play(play_args, &user_config),
        ActionType::List(ref list_args) => mprs_list(list_args, &user_config),
        ActionType::Move(ref move_args) => mprs_move(move_args, &user_config),
        ActionType::Open => mprs_open(&user_config),
    };
}
