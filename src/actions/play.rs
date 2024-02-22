use crate::args::PlayArgs;
use crate::config::UserConfig;
use async_process::Command;
use mprs::utils::{base_dir, list_dir};
use std::fs::{read_dir, DirEntry};
use std::io;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time;
use rand::thread_rng;
use rand::seq::SliceRandom;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn mprs_play(args: &PlayArgs, config: &UserConfig) {
    let mut playlists = list_dir(&config.base_dir);
    let current_song: PathBuf;
    let mut song_queue: Vec<PathBuf>;

    match (&args.query_term, &args.playlist) {
        (q, Some(p)) => {
            let mut selected_playlist = base_dir();
            selected_playlist.push(p.clone());

            if !playlists.contains(&selected_playlist) {
                println!("Playlist {:?} not found", selected_playlist);
                return;
            }
            song_queue = list_dir(&selected_playlist);

            match q {
                Some(query) => {
                    let mut start_index = -1;
                    for (i, x) in song_queue.iter().enumerate() {
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

                    if start_index == -1 {
                        println!("\"{}\" not found in playlist \"{}\"", query, p);
                    }

                    current_song = song_queue.remove(start_index as usize);
                }
                _ => {
                    current_song = song_queue.remove(0);
                }
            }
        }
        (q, None) => {
            song_queue = Vec::new();
            for p in list_dir(&config.base_dir) {
                song_queue.extend(list_dir(&p));
            }

            match q {
                Some(query) => {
                    let mut start_index = -1;
                    for (i, x) in song_queue.iter().enumerate() {
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

                    if start_index == -1 {
                        println!("\"{}\" not found", query);
                    }

                    current_song = song_queue.remove(start_index as usize);
                },
                _ => {
                    current_song = song_queue.remove(0);
                }
            }
        }
    };

    if args.shuffle {
        song_queue.shuffle(&mut thread_rng())
    }
    song_queue.insert(0, current_song);


    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut quit_flag;
    let mut i: i32 = 0;

    while (i as usize) < song_queue.len() {
        let curr_song = song_queue[i as usize].clone();
        println!("Current song: {}", curr_song.as_path().file_name().unwrap().to_str().unwrap());
        let mut cmd = Command::new("afplay")
            .arg(curr_song)
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        quit_flag = false;

        while cmd.try_status().unwrap() == None {
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => {
                        drop(cmd);
                        quit_flag = true;
                        break;
                    }
                    termion::event::Key::Char('n') => {
                        drop(cmd);
                        break;
                    }
                    termion::event::Key::Char('b') => {
                        drop(cmd);
                        i = i - 2;
                        break;
                    }
                    _ => {}
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        }

        i += 1;

        if quit_flag {
            break;
        }
    }
}
