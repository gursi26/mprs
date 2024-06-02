#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod db;
mod mpv;
mod spotdl;
mod track_queue;
mod utils;
mod ui;
mod state;

use db::TrackDB;
// use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use spotdl::{download_track, init_spotify_client, search_tracks};
use state::{state::AppState, tracklist_state::TracklistItem};
use eframe::{egui, NativeOptions};
use tokio::runtime::Runtime;
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

const NUM_SEARCH_RESULTS: u32 = 10;
const MULTIPLE_JUMP_DISTANCE: i32 = 20;

// TODO: Switch to Unix domain sockets for IPC

fn main() {
    init_functions();
    let mut app = AppState::default();
    let mut client = init_spotify_client();
    // let results = search_tracks("rising hope lisa".to_string(), 5, &mut client);
    // download_track(&results.get(0).unwrap().get_url()).wait().unwrap();

    app.trackdb.add_all_tracks(None);

    // app.tracklist_state.items.push(TracklistItem {
    //     id: 1,
    //     name: "Track 1 name".to_string(),
    //     artist: "Artist 1 name".to_string(),
    //     album: "Album 1 name".to_string(),
    //     duration: "10:23".to_string()
    // });
    // app.tracklist_state.items.push(TracklistItem {
    //     id: 1,
    //     name: "Track 1 name".to_string(),
    //     artist: "Artist 1 name".to_string(),
    //     album: "Album 1 name".to_string(),
    //     duration: "10:23".to_string()
    // });
    // app.tracklist_state.items.push(TracklistItem {
    //     id: 1,
    //     name: "Track 1 name".to_string(),
    //     artist: "Artist 1 name".to_string(),
    //     album: "Album 1 name".to_string(),
    //     duration: "10:23".to_string()
    // });
    // app.tracklist_state.items.push(TracklistItem {
    //     id: 1,
    //     name: "Track 1 name".to_string(),
    //     artist: "Artist 1 name".to_string(),
    //     album: "Album 1 name".to_string(),
    //     duration: "10:23".to_string()
    // });
    // app.tracklist_state.items.push(TracklistItem {
    //     id: 1,
    //     name: "Track 1 name".to_string(),
    //     artist: "Artist 1 name".to_string(),
    //     album: "Album 1 name".to_string(),
    //     duration: "10:23".to_string()
    // });
    //

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([960.0, 540.0]),
        ..Default::default()
    };
    // TODO: Also replace with crate name from cargo
    eframe::run_native("mprs", native_options, Box::new(|_| Box::new(app))).unwrap();
    // init_functions();
    // let mut spotify = init_spotify_client();
    // let mut app_state = Arc::new(Mutex::new(AppState::default()));

    // let player_update_state_arc = Arc::clone(&app_state);

    // let rt = Runtime::new().unwrap();
    // let player_update_handle = rt.spawn(async move {
    //     player_handler(player_update_state_arc, PLAYER_HANDLER_TIMEOUT_MS).await;
    // });

    // run(Arc::clone(&app_state), &mut spotify).unwrap();
    // drop(player_update_handle);

    // let curr_rc = Arc::clone(&app_state);
    // let mut curr_app_state = curr_rc.lock().unwrap();

    // match &mut curr_app_state.mpv_child {
    //     Some(c) => {c.kill().unwrap();},
    //     None => {}
    // };
    // exit(0);
}
