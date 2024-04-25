#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod mpv;
mod state;
mod track_queue;
mod utils;

use mpv::{check_next_prev_status, next_track, play_track, update_player_status, wait_for_player};
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

    let mut app_state = AppState {
        mpv_child: Arc::new(Mutex::new(Command::new("ls").spawn().unwrap())),
        paused: Arc::new(Mutex::new(false)),
        next_pressed: Arc::new(Mutex::new(false)),
        prev_pressed: Arc::new(Mutex::new(false)),
        track_queue: Arc::new(Mutex::new(queue)),
    };

    let (c1, c2, c3) = (
        Arc::clone(&app_state.paused),
        Arc::clone(&app_state.next_pressed),
        Arc::clone(&app_state.prev_pressed),
    );

    let player_update_handle = tokio::task::spawn(async {
        update_player_status(c1, c2, c3, 10).await;
    });

    let (c1, c2, c3, c4) = (
        Arc::clone(&app_state.track_queue),
        Arc::clone(&app_state.mpv_child),
        Arc::clone(&app_state.next_pressed),
        Arc::clone(&app_state.prev_pressed),
    );
    let keybind_handle = tokio::task::spawn(async {
        check_next_prev_status(c1, c2, c3, c4, 5).await;
    });
    play_track(Arc::clone(&app_state.mpv_child), &curr_track);

    // cleanup
    wait_for_player(Arc::clone(&app_state.mpv_child));
    std::mem::drop(player_update_handle);
}
