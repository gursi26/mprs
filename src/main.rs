#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod mpv;
mod state;
mod track_queue;
mod utils;

use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use state::AppState;
use stopwatch::Stopwatch;
use std::{
    fs::create_dir_all,
    path::PathBuf,
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use track_queue::TrackQueue;
// Import init_files function from utils module
crate:: utils::init_files as init_files;
use utils::init_files as init_files;

const MUSIC_DIR: &str = "/Users/stephenscott/mprs-tracks";
const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";

const PLAYER_HANDLER_TIMEOUT_MS: u64 = 20;
const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;

#[tokio::main]
async fn main() {
    init_files();

    let mut queue = TrackQueue::new();
    queue.add_to_reg_queue(PathBuf::from_str("/Users/stephenscott/mprs-music/liked/Set Fire to the Rain.mp3").unwrap());
    queue.add_to_reg_queue(PathBuf::from_str("/Users/stephenscott/mprs-music/liked/Sunny Sunday [gcCyiUuCjqQ].mp3").unwrap());
    queue.play_next(PathBuf::from_str("/Users/stephenscott/mprs-music/liked/Olivia Dean - Dive (Official Video) [NM4e606yFJg].mp3").unwrap());

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
