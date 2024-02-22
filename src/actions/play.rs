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
use std::time::Instant;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

static TIME_UNTIL_BACK_SELF: u64 = 3;

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
    let mut quit_flag: bool = false;
    let mut i: i32 = 0;
    let mut curr_volume = 1.0;
    let mut curr_speed = 1.0;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    while (i as usize) < song_queue.len() && !quit_flag {
        // Get and print current song
        let curr_song = &song_queue[i as usize];
        println!("Current song: {}", curr_song.as_path().file_name().unwrap().to_str().unwrap());

        // append current song to sink
        sink.append(Decoder::new(BufReader::new(File::open(curr_song).unwrap())).unwrap());
        sink.set_volume(curr_volume);
        sink.set_speed(curr_speed);
        let start_time = Instant::now();

        while !sink.empty() {
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Char('q') => {
                        sink.stop();
                        quit_flag = true;
                        break;
                    }
                    termion::event::Key::Char('n') => {
                        sink.stop();
                        break;
                    }
                    termion::event::Key::Char('b') => {
                        sink.stop();
                        i -= 1;
                        if start_time.elapsed().as_secs() < TIME_UNTIL_BACK_SELF {
                            i -= 1;
                        }
                        break;
                    }
                    termion::event::Key::Char('p') => {
                        if sink.is_paused() {
                            sink.play();
                        } else {
                            sink.pause();
                        }
                    }
                    termion::event::Key::Char('+') => {
                        curr_volume += 0.1;
                        sink.set_volume(curr_volume);
                    }
                    termion::event::Key::Char('-') => {
                        curr_volume -= 0.1;
                        sink.set_volume(curr_volume);
                    }
                    termion::event::Key::Right => {
                        curr_speed += 0.1;
                        sink.set_speed(curr_speed);
                    }
                    termion::event::Key::Left => {
                        curr_speed -= 0.1;
                        sink.set_speed(curr_speed);
                    }
                    termion::event::Key::Up => {
                        curr_speed = 1.0;
                        sink.set_speed(curr_speed);
                    }
                    _ => {}
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        }
        i += 1;
    }
}
