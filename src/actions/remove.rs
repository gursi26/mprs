use crate::args::RemoveArgs;
use crate::config::UserConfig;
use std::io::{stdout, Write};
use mprs::utils;
use mprs::utils::print_table;
use std::fs;
use std::io;  // might be necessary
use std::path::Path;
use prettytable::{Table, Cell, Row};  // should this be in utils?

pub fn mprs_remove(args: &RemoveArgs, config: &UserConfig) {
    // NOTE: I probably confirm that the playlist exists before attempting to remove a song from it.

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
    // let playlist_path = &config.base_dir;
    playlist_path.push(&args.playlist);
    println!("Playlist path: {:?}", playlist_path);

    // Declare index representing which song to delete (assigned via user input if no query term is given).
    let id_idx: i32;

    // Print all songs in playlist and let user decide which to remove.
    if query == "None" {
        let mut table = Table::new();
        let mut row_vec: Vec<Cell>;

        // IO copied from add.rs
        print!("Select song to remove by number : ");
        let _ = stdout().flush();

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        let id_idx: i32 = input_string.trim().parse().unwrap();

    }

    // Using utils
    let paths = utils::list_dir(&playlist_path.to_owned());
    println!("\nSongs in playlist \"{}\": ", args.playlist);
    for path in paths {
        println!("Name: {}", path.display())
    }

    // TODO: figure out how to print the number of songs in a playlist if no query term is given

    // Example remove command:
    // Ref: https://doc.rust-lang.org/std/fs/fn.remove_file.html#:~:text=Function%20std%3A%3Afs%3A%3Aremove_file&text=Removes%20a%20file%20from%20the,descriptors%20may%20prevent%20immediate%20removal).
    // fs::remove_file("liked/1.mp3").unwrap();

    // paths[(id_idx - 1) as usize];
    return;

}
