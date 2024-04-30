#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod mpv;
mod state;
mod track_queue;
mod utils;
mod spotdl;
mod db;

use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use db::TrackDB;
use spotdl::{download_track, init_spotify_client, search_tracks};
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

// TODO: Write tui code
// TODO: Integrate trackdb code
// TODO: Switch to Unix domain sockets for IPC

#[tokio::main]
async fn main() {
    init_files();
    check_spotdl_installed();

    let mut spotify = init_spotify_client();
    let results = search_tracks(String::from("visit to hida"), 5, &mut spotify).await;
    dbg!(&results[0]);
    download_track(&results[0]);

    let results = search_tracks(String::from("dream lantern"), 5, &mut spotify).await;
    dbg!(&results[0]);
    download_track(&results[0]);

    let mut db = TrackDB::init();
    db.add_all_tracks(Some("new_playlist".to_string()));

    let results = search_tracks(String::from("sparkle"), 5, &mut spotify).await;
    dbg!(&results[0]);
    download_track(&results[0]);
    db.add_all_tracks(None);

    dbg!(&db);

    dbg!("removing...");
    db.remove_track(2);
    dbg!(&db);

    // let mut queue = TrackQueue::new();
    // queue.add_to_reg_queue(PathBuf::from_str("/Users/gursi/mprs-music/rn/kaw2.mp3").unwrap());
    // queue.add_to_reg_queue(PathBuf::from_str("/Users/gursi/mprs-music/rn/Visit to Hida.mp3").unwrap());
    // queue.play_next(PathBuf::from_str("/Users/gursi/mprs-music/rn/YUI - again.mp3").unwrap());

    // let mut app_state = Arc::new(Mutex::new(AppState {
    //     track_queue: queue,
    //     ..Default::default()
    // }));

    // let mut as_g = app_state.lock().unwrap();
    // let rc_clone = Arc::clone(&app_state);

    // initialize_player(&mut as_g);
    // drop(as_g);

    // let player_update_handle = tokio::task::spawn(async move {
    //     player_handler(rc_clone, PLAYER_HANDLER_TIMEOUT_MS).await;
    // }).await.unwrap();
}
