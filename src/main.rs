#![allow(dead_code, unused_mut, unused_variables, unused_imports)]
#[macro_use] extern crate prettytable;

mod args;
mod config;
mod ytdlp;
use std::path::PathBuf;

use prettytable::{Table, Row, Cell};

use args::*;
use clap::Parser;
use config::{init_config, parse_config_file, UserConfig};
use ytdlp::{search_ytdlp, download};
use std::io::{stdout, Write};


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

fn print_table(table_content: &Vec<(String, String, String, String)>) {
    let mut table = Table::new();
    table.add_row(row!["#", "Name", "Creator", "Duration", "Upload Date"]);
    for (i, x) in table_content.iter().enumerate() {
        table.add_row(row![i + 1, x.0, x.1, x.2, x.3]);
    }
    table.printstd();
}

fn mprs_add(args: &AddArgs, config: &UserConfig) {
    // println!("{:?}", config);
    // println!("{:?}", args);
    
    let (id_vec, results_vec) = search_ytdlp(&args.query_term, args.count);
    print_table(&results_vec);

    print!("Select song by number : ");
    let _ = stdout().flush();

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    let id_idx: i32 = input_string.trim().parse().unwrap();

    let mut save_path = config.base_dir.clone();
    save_path.push(&args.playlist);

    if download(&id_vec[(id_idx - 1) as usize], &config.audio_format, &save_path) {
        println!("Successfully downloaded.");
    }

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
