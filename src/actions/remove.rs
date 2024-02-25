use crate::actions::{play, remove};
use crate::args::RemoveArgs;
use crate::config::UserConfig;
use std::io::{stdout, Write, stdin};
use mprs::utils::{print_table, list_dir};
use std::fs::remove_file;
use std::io;
use std::path::{Path, PathBuf};

pub fn mprs_remove(args: &RemoveArgs, config: &UserConfig) 
{
    println!("\nDEBUG INFO:");
    println!("{:?}", config);
    println!("{:?}", args);

    println!(
        "Query: {:?}\nPlaylist: {:?}",
        args.track, args.playlist
    );

    // Get all playlists in base directory.
    let mut playlists = list_dir(&config.base_dir);
    let mut remove_options = vec![];

    match (&args.track, &args.playlist) {
        (q, Some(playlist)) => {
            let mut playlist_path = config.base_dir.clone();
            playlist_path.push(&playlist.clone());

            // Catch invalid user input
            if !playlist_path.is_dir() {
                println!("Playlist {:?} does not exist.\nThere is thus nothing to remove.", playlist_path);
                return;
            }

            for x in list_dir(&playlist_path) {
                match q {
                    Some(query) => {
                        if x.as_path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .contains(&query.to_lowercase()[..])
                        {
                            remove_options.push(x);
                        }
                    }
                    None => {
                        remove_options.push(x);
                    }
                }
            }
        }
        (q, None) => {
            for playlist in playlists.iter() {
                for track in list_dir(&playlist) {
                    match q {
                        Some(query) => {
                            if track.as_path()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_lowercase()
                                .contains(&query.to_lowercase()[..])
                            {
                                remove_options.push(track);
                            }
                        }
                        None => {
                            remove_options.push(track);
                        }
                    }
                }
            }
        }
    }

    if remove_options.len() == 0 {
        println!("No matching tracks found.");
        return;
    } 
    if remove_options.len() == 1 {
        // Do not print table of options if there is only one option.
        let removed_song = &remove_options[0];

        let removed_song_title = Path::new(removed_song.file_stem().unwrap()).file_name().unwrap();
        let removed_from_playlist = Path::new(removed_song.parent().unwrap()).file_name().unwrap();
        println!("Removed {:?} from playlist {:?}.", removed_song_title, removed_from_playlist);

        remove_file(removed_song.to_owned()).unwrap();
        return;
    }

    // Declare index representing which song to delete (assigned via user input if no query term is given).
    let id_idx: i32;

    let mut table_content = vec![vec!["Song Title".to_string()]];
    for path in remove_options.iter() {
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

    let removed_song = &remove_options[(id_idx - 1) as usize];

    let removed_song_title = Path::new(removed_song.file_stem().unwrap()).file_name().unwrap();
    let removed_from_playlist = Path::new(removed_song.parent().unwrap()).file_name().unwrap();
    println!("Removed {:?} from playlist {:?}.", removed_song_title, removed_from_playlist);

    remove_file(removed_song.to_owned()).unwrap();
}
