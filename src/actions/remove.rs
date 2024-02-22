use crate::args::RemoveArgs;
use crate::config::UserConfig;
use std::io::{stdout, Write};
use mprs::utils::print_table;
use std::fs::remove_file;
use std::fs::read_dir;
// these are for read_dir
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn mprs_remove(args: &RemoveArgs, config: &UserConfig) {
    println!("\nDEBUG INFO:");
    println!("{:?}", config);
    println!("{:?}", args);

    // Unwrap args.query_term
    let query = args.query_term.as_deref().unwrap_or("None");

    println!(
        "Query: {}\nPlaylist: {}",
        query, args.playlist
    );

    let mut playlist_path = config.base_dir.clone();
    playlist_path.push(&args.playlist);
    println!("Playlist path: {:?}", playlist_path);

    // IO copied from add.rs
    print!("Select song by number : ");
    let _ = stdout().flush();

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    let id_idx: i32 = input_string.trim().parse().unwrap();

    // TODO: figure out how to print the number of songs in a playlist if no query term is given

}
