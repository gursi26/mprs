use std::fs;
use mprs::utils::list_dir;
use chrono::{DateTime, Local, Utc};

use crate::args::ListArgs;
use crate::config::UserConfig;
use crate::utils::{print_table, base_dir, get_duration};

pub fn mprs_list(args: &ListArgs, config: &UserConfig) {
    let mut table: Vec<Vec<String>> = Vec::new();
    if !args.list_all {
        match &args.playlist {
            Some(p) => {
                let mut playlist_path = base_dir();
                playlist_path.push(p);
                if !playlist_path.is_dir() {
                    println!("Playlist {:?} not found", playlist_path);
                    return;
                }

                let songs = list_dir(&playlist_path);

                table.push(vec!["Song Name", "Duration", "Date Added"].iter().map(|&x| x.to_string()).collect());
                for song_path in songs {
                    let mut curr_vec = Vec::new();
                    let seconds = get_duration(&song_path);
                    curr_vec.push(song_path.clone().as_path().file_name().unwrap().to_str().unwrap().to_string());
                    curr_vec.push(format!("{}:{:0>2}", seconds / 60, seconds - ((seconds / 60) * 60)));
                    let metadata = fs::metadata(&song_path).unwrap();
                    if let Ok(time) = metadata.created() {
                        let datetime: DateTime<Utc> = time.clone().into();
                        curr_vec.push(datetime.to_string());
                    } else {
                        curr_vec.push(String::from(""));
                    }
                    table.push(curr_vec);
                }
            },
            None => {
                let playlist_paths = list_dir(&config.base_dir);
                let song_counts: Vec<usize> = playlist_paths.iter().map(|x| list_dir(x).len()).collect();
                let mut playlists = Vec::new();
                let mut created_dates = Vec::new();
                for p in playlist_paths {
                    let metadata = fs::metadata(&p).unwrap();
                    if let Ok(time) = metadata.created() {
                        let datetime: DateTime<Utc> = time.clone().into();
                        created_dates.push(datetime.to_string());
                    } else {
                        created_dates.push(String::from(""));
                    }
                    playlists.push(p.as_path().file_name().unwrap().to_str().unwrap().to_string());
                }
                table.push(vec!["Playlist Name", "Song Count", "Date Created"].iter().map(|&x| x.to_string()).collect());
                for i in 0..playlists.len() {
                    table.push(vec![playlists[i].clone(), song_counts[i].to_string(), created_dates[i].clone()]);
                }
            }
        }
    } else {
        let playlist_paths = list_dir(&config.base_dir);
        for playlist_path in playlist_paths {
            table.push(vec!["Song Name", "Playlist", "Duration", "Date Added"].iter().map(|&x| x.to_string()).collect());
            let playlist_name = playlist_path.clone().as_path().file_name().unwrap().to_str().unwrap().to_string();
            let song_paths = list_dir(&playlist_path);
            for song_path in song_paths {
                let mut curr_vec = Vec::new();
                let seconds = get_duration(&song_path);
                curr_vec.push(song_path.clone().as_path().file_name().unwrap().to_str().unwrap().to_string());
                curr_vec.push(playlist_name.clone());
                curr_vec.push(format!("{}:{:0>2}", seconds / 60, seconds - ((seconds / 60) * 60)));
                let metadata = fs::metadata(&song_path).unwrap();
                if let Ok(time) = metadata.created() {
                    let datetime: DateTime<Utc> = time.clone().into();
                    curr_vec.push(datetime.to_string());
                } else {
                    curr_vec.push(String::from(""));
                }
                table.push(curr_vec);

            }
        }

    }

    print_table(&table);
}
