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

pub fn wait_for_player(mpv_child: Arc<Mutex<Child>>) {
    let mut child = mpv_child.lock().unwrap();
    child.wait().unwrap();
}

// TODO: replace track path with track id and lookup in db
pub fn play_track(mpv_child: Arc<Mutex<Child>>, track_path: &PathBuf) {
    let mut child = mpv_child.lock().unwrap();
    child.kill();

    *child = Command::new("mpv")
        .arg(track_path.to_str().unwrap())
        .arg("--no-terminal")
        .arg(format!(
            "--script={}",
            get_luascript_path().to_str().unwrap()
        ))
        .spawn()
        .unwrap();
}

pub fn next_track(q: Arc<Mutex<TrackQueue>>, mpv_child: Arc<Mutex<Child>>) {
    let mut q_g = q.lock().unwrap();

    q_g.next_track();
    play_track(mpv_child, q_g.get_curr_track_path());
}

pub async fn update_player_status(
    paused: Arc<Mutex<bool>>,
    next_pressed: Arc<Mutex<bool>>,
    prev_pressed: Arc<Mutex<bool>>,
    sleep_millis: u64,
) {
    let ipc_fp = get_ipc_path();
    let mut prev_file_contents = String::new();
    loop {
        let file_contents = read_to_string(&ipc_fp).unwrap();

        if file_contents == prev_file_contents {
            sleep(Duration::from_millis(sleep_millis));
            continue;
        } else {
            prev_file_contents = file_contents;
        }

        dbg!(&prev_file_contents);

        let (mut paused_g, mut next_pressed_g, mut prev_pressed_g) = (
            paused.lock().unwrap(),
            next_pressed.lock().unwrap(),
            prev_pressed.lock().unwrap(),
        );

        let mut split_contents = prev_file_contents.split_whitespace();

        *paused_g = parse_bool(split_contents.next().unwrap());
        *next_pressed_g = parse_bool(split_contents.next().unwrap());
        *prev_pressed_g = parse_bool(split_contents.next().unwrap());

        drop(paused_g);
        drop(next_pressed_g);
        drop(prev_pressed_g);
        sleep(Duration::from_millis(sleep_millis));
    }
}

pub async fn check_next_prev_status(
    q: Arc<Mutex<TrackQueue>>,
    mpv_child: Arc<Mutex<Child>>,
    next_pressed: Arc<Mutex<bool>>,
    prev_pressed: Arc<Mutex<bool>>,
    sleep_millis: u64,
) {
    loop {
        let (mut next_pressed_g, mut prev_pressed_g) =
            (next_pressed.lock().unwrap(), prev_pressed.lock().unwrap());

        if *next_pressed_g {
            next_track(Arc::clone(&q), Arc::clone(&mpv_child));
            *next_pressed_g = false;
        }

        sleep(Duration::from_millis(sleep_millis));
    }
}
