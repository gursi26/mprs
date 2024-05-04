#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod db;
mod mpv;
mod spotdl;
mod state;
mod track_queue;
mod utils;
mod tui;

use db::TrackDB;
use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use spotdl::{download_track, init_spotify_client, search_tracks};
use state::AppState;
use tui::run;
use std::{
    fs::create_dir_all,
    path::PathBuf,
    process::{exit, Command},
    str::FromStr,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use stopwatch::Stopwatch;
use track_queue::TrackQueue;
use utils::init_functions;

const MUSIC_DIR: &str = "mprs-tracks";
const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";

const PLAYER_HANDLER_TIMEOUT_MS: u64 = 20;
const UI_SLEEP_DURATION_MS: u64 = 10;
const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;
const KEY_INPUT_POLL_TIMEOUT_MS: u64 = 250;
const NOTIFICATION_TIMEOUT_S: u64 = 3;

const MULTIPLE_JUMP_DISTANCE: i32 = 20;

// TODO: Write tui code
// TODO: Switch to Unix domain sockets for IPC

#[tokio::main]
async fn main() {
    init_functions();
    let mut spotify = init_spotify_client();
    let mut app_state = Arc::new(Mutex::new(AppState::default()));

    let player_update_state_arc = Arc::clone(&app_state);

    let player_update_handle = tokio::task::spawn(async move {
        player_handler(player_update_state_arc, PLAYER_HANDLER_TIMEOUT_MS).await;
    });

    let a = Arc::clone(&app_state);
    let mut l = a.lock().unwrap();
    l.track_db.add_all_tracks(Some("rn".to_string()));
    drop(l);

    run(Arc::clone(&app_state)).await.unwrap();
    drop(player_update_handle);

    let curr_rc = Arc::clone(&app_state);
    let mut curr_app_state = curr_rc.lock().unwrap();

    match &mut curr_app_state.mpv_child {
        Some(c) => {c.kill().unwrap();},
        None => {}
    };
    exit(0);
}
