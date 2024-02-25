use std::path::PathBuf;

use std::io::{stdout, Write};

use crate::args::MoveArgs;
use crate::config::UserConfig;
use crate::utils::*;

pub fn mprs_move(args: &MoveArgs, config: &UserConfig) {
    let mut dest_path = base_dir();
    dest_path.push(&args.dest_playlist);

    if !dest_path.is_dir() {
        println!("Playlist {:?} not found", dest_path);
        return;
    }

    let mut track_paths: Vec<PathBuf> = Vec::new();

    match &args.source_playlist {
        Some(source) => {
            let mut source_path = base_dir();
            source_path.push(source);

            if !source_path.is_dir() {
                println!("Playlist {:?} not found", source_path);
                return;
            }

            track_paths = list_dir(&source_path);
        }
        None => {
            let playlist_paths = list_dir(&base_dir());
            for playlist_path in playlist_paths.iter() {
                track_paths.append(&mut list_dir(playlist_path));
            }
        }
    }

    track_paths.retain(|x| {
        get_track_name(x)
            .to_lowercase()
            .contains(&args.track.to_lowercase()[..])
    });

    let selected_track;

    match track_paths.len() {
        0 => {
            println!("Track \"{}\" not found", &args.track);
            return;
        },
        1 => {
            selected_track = &track_paths[0];
        },
        _ => {
            let mut table = Vec::new();
            table.push(vec!["Track name", "Artist", "Playlist", "Duration", "Date Added"].iter().map(|x| x.to_string()).collect());
            for p in track_paths.iter() {
                table.push(get_track_information(p));
            }
            print_table(&table);

            print!("Select song by number : ");
            let _ = stdout().flush();

            let mut input_string = String::new();
            std::io::stdin().read_line(&mut input_string).unwrap();
            let id_idx: i32 = input_string.trim().parse().unwrap();

            selected_track = &track_paths[(id_idx - 1) as usize];
        }
    }

    match mv_cpy_file(selected_track, &dest_path, args.copy) {
        Ok(()) => {
            if args.copy {
                println!("Successfully copied from {:?} to {:?}", selected_track, &dest_path);
            } else {
                println!("Successfully moved from {:?} to {:?}", selected_track, &dest_path);
            }
        },
        Err(_) => {
            if args.copy {
                println!("Error! Could not copy track!");
            } else {
                println!("Error! Could not move track!");
            }
        }
    }
}

fn mv_cpy_file(src_path: &PathBuf, dest_dir: &PathBuf, copy: bool) -> std::io::Result<()> {
    let song_name = get_track_name(src_path);
    let mut dest_path = dest_dir.clone();
    dest_path.push(song_name);
    if copy {
        std::fs::copy(src_path, dest_path)?;
    } else {
        std::fs::rename(src_path, dest_path)?;
    }
    Ok(())
}
