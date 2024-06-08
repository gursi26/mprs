#![allow(dead_code, unused_mut, unused_variables, unused_imports)]

mod db;
mod mpv;
mod spotdl;
mod track_queue;
mod utils;
mod ui;
mod state;

use db::TrackDB;
use egui_extras::install_image_loaders;
use mpv::player_handler;
// use mpv::{initialize_player, next_track, play_track, player_handler, wait_for_player};
use spotdl::{download_track, init_spotify_client, search_tracks};
use state::{filter_state::F1State, state::{AppState, AppStateWrapper}, tracklist_state::TracklistItem};
use eframe::{egui::{self, FontData, FontFamily}, NativeOptions};
use tokio::runtime::Runtime;
use std::{
    collections::BTreeMap, fs::create_dir_all, path::PathBuf, process::{exit, Command}, str::FromStr, sync::{Arc, Mutex}, thread::sleep, time::Duration
};
use stopwatch::Stopwatch;
use track_queue::TrackQueue;
use utils::init_functions;

const MUSIC_DIR: &str = "mprs-tracks";
const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";

const PLAYER_HANDLER_TIMEOUT_MS: u64 = 20;
const UI_SLEEP_DURATION_MS: u64 = 1000;
const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;
const KEY_INPUT_POLL_TIMEOUT_MS: u64 = 250;
const NOTIFICATION_TIMEOUT_S: u64 = 3;

const NUM_SEARCH_RESULTS: u32 = 10;
const MULTIPLE_JUMP_DISTANCE: i32 = 20;

// TODO: Unicode font rendering
// TODO: Write functions unshuffle add tracks to queue (add in order and skip until curr track
// reached)
// TODO: Clean up code and create functions for common operations (accessing curr track list)
// TODO: Create playlist with spotify link to pull all tracks (or add import playlist button to add
// multiple tracks to existing playlist)
// TODO: Add feature to edit playlist name
// TODO: Put some hardcoded values into constants
// TODO: Add option to normalize track volume
// TODO: Set a consistent audio format for song downloads with spotdl
// TODO: Store spotify trackid with each track to identify when a track is already downloaded, so
// adding the same track to multiple playlists does not do multiple downloads
// TODO: Find out how to change column scheme to black/dark blue palete and not beige ish
// TODO: Improve first run experience by adding options to select track download dir and manage
// permission stuff
// TODO: Add playlist stats in bottom bar (total number of tracks + total duration)
// TODO: Maybe add a script to download deps (spotdl/python which is needed for spotdl)
// TODO: VISUALIZER
// TODO: Make queue and visualizer windows optional (keybind toggle?)
// TODO: Replace empty table on empty playlist with no tracks message
// TODO: Add a way to see the current queue and played tracks history
// TODO: Move search for add tracks to popup window
// TODO: Add spacing between columns in tracklist
// TODO: Only update screen when user is focused on it
// TODO: Replace package name/version/window title with crate name and version from cargo
// TODO: Make button coloring and size more consistent
// TODO: Make panel sizing a fraction of window sizing for more consistency (or allow resizing)
// TODO: Optimize update functions to be minimal so that frames can be rendered with minimal cpu
// usage
// TODO: Add pausing with space bar
// TODO: Change dock icon
// NOTE: Maybe add album covers to tracklist? (May affect performance poorly)

fn main() {
    init_functions();
    let mut app_inner = AppState::default();

    let mut app = AppStateWrapper { app_state: Arc::new(Mutex::new(app_inner))};
    
    let app_state_rc = Arc::clone(&app.app_state);
    
    let rt = Runtime::new().unwrap();
    let player_update_handle = rt.spawn(async move {
        player_handler(app_state_rc, PLAYER_HANDLER_TIMEOUT_MS).await;
    });

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([960.0, 540.0]),
        ..Default::default()
    };

    // TODO: Also replace with crate name from cargo
    eframe::run_native("mprs", native_options, Box::new(|cc| {
        Box::new(app)
    })).unwrap();
}
