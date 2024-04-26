use crate::track_queue::TrackQueue;
use crate::utils::{get_ipc_path, parse_bool};
use crate::{state::AppState, utils::get_luascript_path};
use std::fs::read_to_string;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{
    mem::drop,
    path::PathBuf,
    process::Child,
    sync::{Arc, Mutex},
};

pub fn kill_track(mpv_child: Arc<Mutex<Child>>) {
    let mut child = mpv_child.lock().unwrap();
    child.kill().unwrap();
}

pub fn wait_for_player(app_state: Arc<Mutex<AppState>>) {
    let mut app_state_rc = app_state.lock().unwrap();
    app_state_rc.mpv_child.wait().unwrap();
}

pub fn initialize_player(app_state: &mut AppState) {
    app_state.mpv_child.kill().unwrap();
    let curr_track = app_state.track_queue.get_curr_track_path().clone();
    play_track(app_state, &curr_track);
}

// TODO: replace track path with track id and lookup in db
pub fn play_track(app_state: &mut AppState, track_path: &PathBuf) {
    let dbg_str = format!("Now playing: {}", track_path.to_str().unwrap());
    dbg!(dbg_str);

    app_state.mpv_child.kill().unwrap();

    app_state.mpv_child = Command::new("mpv")
        .arg(track_path.to_str().unwrap())
        .arg("--no-terminal")
        .arg(format!(
            "--script={}",
            get_luascript_path().to_str().unwrap()
        ))
        .spawn()
        .unwrap();
}

pub fn next_track(app_state: &mut AppState) {
    app_state.track_queue.next_track();
    let track_ref = app_state.track_queue.get_curr_track_path().clone();
    play_track(
        app_state,
        &track_ref
    );
}

pub fn prev_track(app_state: &mut AppState) {
    app_state.track_queue.prev_track();
    let track_ref = app_state.track_queue.get_curr_track_path().clone();
    play_track(
        app_state,
        &track_ref
    );
}

pub async fn player_handler(app_state: Arc<Mutex<AppState>>, sleep_millis: u64) {
    let ipc_fp = get_ipc_path();
    let mut prev_file_contents = String::new();
    loop {
        let mut app_state_rc = app_state.lock().unwrap();

        // check if current track is over and play next next track if so
        if let Some(status) = app_state_rc.mpv_child.try_wait().unwrap() {
            next_track(&mut app_state_rc);
        }

        let file_contents = read_to_string(&ipc_fp).unwrap();
        if file_contents == prev_file_contents {
            sleep(Duration::from_millis(sleep_millis));
            continue;
        } else {
            prev_file_contents = file_contents;
        }

        let mut split_contents = prev_file_contents.split_whitespace();

        app_state_rc.paused = parse_bool(split_contents.next().unwrap());

        // check if next track button was pressed
        if parse_bool(split_contents.next().unwrap()) {
            next_track(&mut app_state_rc);
        }

        // check if prev track button was pressed
        if parse_bool(split_contents.next().unwrap()) {
            prev_track(&mut app_state_rc);
        }

        sleep(Duration::from_millis(sleep_millis));
    }
}
