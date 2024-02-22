use crate::actions::remove;
use crate::args::RemoveArgs;
use crate::config::UserConfig;
use std::io::{stdout, Write, stdin};
use mprs::utils;
use mprs::utils::{print_table, list_dir};
use std::fs::remove_file;
use std::io;
use std::path::Path;
use prettytable::{Table, Cell, Row};  // should this be in utils?

pub fn mprs_remove(args: &RemoveArgs, config: &UserConfig) 
{
    println!("\nDEBUG INFO:");
    println!("{:?}", config);
    println!("{:?}", args);

    // Unwrap args.query_term
    let query = args.query_term.as_deref().unwrap_or("None");

    println!(
        "Query: {}\nPlaylist: {}",
        query, args.playlist
    );

    // Confirm that the playlist exists before attempting to remove a song from it.
    let mut playlists = list_dir(&config.base_dir);

    let mut selected_playlist = config.base_dir.clone();
    selected_playlist.push(&args.playlist.clone());
    
    if !playlists.contains(&selected_playlist) {
        println!("Playlist {:?} does not exist.\nThere is thus nothing to remove.", selected_playlist);
        return;
    }

    println!("Playlist path: {:?}", selected_playlist);

    // Using utils, get path of all songs in playlist and store in a vector.
    let paths = list_dir(&selected_playlist.to_owned());

    // Declare index representing which song to delete (assigned via user input if no query term is given).
    let id_idx: i32;

    // Print all songs in playlist and let user decide which to remove.
    if query == "None" {
        let mut table_content = vec![vec!["Song Title".to_string()]];
        for path in &paths {
            let song_title = Path::new(path.file_stem().unwrap()).file_name().unwrap().to_os_string();
            table_content.push(vec![song_title.to_str().unwrap().to_string()]);
        }
        print_table(&table_content);

        // Get user input for which song to remove.
        let mut input_string = String::new();
        print!("Select song to remove by number : ");
        let _ = stdout().flush();
        std::io::stdin().read_line(&mut input_string).unwrap();
        // Convert user input to i32 and store in id_idx.
        id_idx = input_string.trim().parse().unwrap();
    }

    // Set id_idx to remove first song from playlist matching query term.
    else {  // User has provided a query term indicating a song to remove
        let mut start_index = -1;
        for (i, x) in paths.iter().enumerate() {
            if x.as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .contains(&query.to_lowercase()[..])
            {
                start_index = i as i32;
            }
        }
        // Set id_idx accordingly
        id_idx = start_index;

        if start_index == -1 {
            println!("\"{}\" not found in playlist \"{:?}\"", query, selected_playlist);
        }
    }

    let removed_song = &paths[(id_idx - 1) as usize];
    println!("Removed song: {:?}", removed_song);

    remove_file(removed_song.to_owned()).unwrap();
}
