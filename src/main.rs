#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod mpv;
mod state;
mod track_queue;
mod utils;
mod spotdl;

use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use spotdl::{init_spotify_client, search_tracks};
use state::AppState;
use stopwatch::Stopwatch;
use std::{
    fs::create_dir_all,
    path::PathBuf,
    process::{exit, Command},
    str::FromStr,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use track_queue::TrackQueue;
use utils::{check_spotdl_installed, init_files};

const MUSIC_DIR: &str = "mprs-tracks";
const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";

const PLAYER_HANDLER_TIMEOUT_MS: u64 = 20;
const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;

#[tokio::main]
async fn main() {
    init_files();
    check_spotdl_installed();

    let mut spotify = init_spotify_client();
    let results = search_tracks(String::from("visit to hida"), 5, &mut spotify).await;

    let mut queue = TrackQueue::new();
    queue.add_to_reg_queue(PathBuf::from_str("/Users/gursi/mprs-music/rn/kaw2.mp3").unwrap());
    queue.add_to_reg_queue(PathBuf::from_str("/Users/gursi/mprs-music/rn/Visit to Hida.mp3").unwrap());
    queue.play_next(PathBuf::from_str("/Users/gursi/mprs-music/rn/YUI - again.mp3").unwrap());

    let mut app_state = Arc::new(Mutex::new(AppState {
        mpv_child: Command::new("ls").spawn().unwrap(),
        paused: false,
        track_queue: queue,
        track_clock: Stopwatch::new()
    }));

    let mut as_g = app_state.lock().unwrap();
    let rc_clone = Arc::clone(&app_state);

    initialize_player(&mut as_g);
    drop(as_g);

    let player_update_handle = tokio::task::spawn(async move {
        player_handler(rc_clone, PLAYER_HANDLER_TIMEOUT_MS).await;
    }).await.unwrap();
}
