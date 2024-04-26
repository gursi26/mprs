#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod mpv;
mod state;
mod track_queue;
mod utils;

use mpv::{next_track, play_track, update_player_status, wait_for_player};
use state::AppState;
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
use utils::init_files;

const MUSIC_DIR: &str = "/Users/gursi/mprs-tracks";
const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";

// TODO: Change all functions to take Arc<Mutex<AppState>>
// TODO: Do next and prev handling in same function that reads from ipc file, maybe remove
// next and prev pressed attribtues from AppState

#[tokio::main]
async fn main() {
    init_files();

    let mut queue = TrackQueue::new();
    queue.append(PathBuf::from_str("/Users/gursi/mprs-music/rn/kaw.mp3").unwrap());
    queue.append(PathBuf::from_str("/Users/gursi/mprs-music/rn/Visit to Hida.mp3").unwrap());
    queue.curr_idx = 0;
    let curr_track = queue.get_curr_track_path().clone();

    let mut app_state = Arc::new(Mutex::new(AppState {
        mpv_child: Command::new("ls").spawn().unwrap(),
        paused: false,
        next_pressed: false,
        prev_pressed: false,
        track_queue: queue,
    }));

    let mut as_g = app_state.lock().unwrap();
    let rc_clone = Arc::clone(&app_state);

    play_track(&mut as_g, &curr_track);
    drop(as_g);

    let player_update_handle = tokio::task::spawn(async move {
        update_player_status(rc_clone, 10).await;
    }).await.unwrap();
}
