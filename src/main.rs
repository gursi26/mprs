#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod db;
mod mpv;
mod spotdl;
mod state;
mod track_queue;
mod utils;

use db::TrackDB;
use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use spotdl::{download_track, init_spotify_client, search_tracks};
use state::AppState;
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
const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;

// TODO: Write tui code
// TODO: Switch to Unix domain sockets for IPC

#[tokio::main]
async fn main() {
    init_functions();

    let mut spotify = init_spotify_client();
    let mut app_state = Arc::new(Mutex::new(AppState::default()));
    let player_update_state_arc = Arc::clone(&app_state);
    let mut curr_app_state = app_state.lock().unwrap();

    let results = search_tracks(String::from("visit to hida"), 5, &mut spotify).await;
    download_track(&results[0]);
    let results = search_tracks(String::from("dream lantern"), 5, &mut spotify).await;
    download_track(&results[0]);
    let results = search_tracks(String::from("Gurenge"), 5, &mut spotify).await;
    download_track(&results[0]);
    curr_app_state.track_db.add_all_tracks(Some("playlist1".to_string()));

    curr_app_state.track_db.change_playlist(1, "playlist2".to_string());
    curr_app_state.track_db.change_title(1, "lmao wtf".to_string());
    curr_app_state.track_db.change_artists(1, Some(vec!["RADWIMPS".to_string(), "LiSA".to_string()]));
    
    let results = search_tracks(String::from("sparkle"), 5, &mut spotify).await;
    download_track(&results[0]);
    curr_app_state.track_db.add_all_tracks(None);

    curr_app_state.track_db.remove_playlist("playlist1".to_string());

    // curr_app_state.track_queue.add_to_reg_queue(1);
    // curr_app_state.track_queue.add_to_reg_queue(2);
    // curr_app_state.track_queue.play_next(3);

    // initialize_player(&mut curr_app_state);
    // drop(curr_app_state);

    // let player_update_handle = tokio::task::spawn(async move {
    //     player_handler(player_update_state_arc, PLAYER_HANDLER_TIMEOUT_MS).await;
    // }).await.unwrap();
}
