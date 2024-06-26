use log::debug;
use stopwatch::Stopwatch;
use crate::track_queue::TrackQueue;
use crate::utils::{get_album_cover, get_ipc_path, parse_bool};
use crate::PREV_SAME_TRACK_TIMEOUT_S;
use crate::utils::get_luascript_path;
use crate::state::state::AppState;
use std::fs::read_to_string;
use std::process::{exit, Command};
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

// pub fn wait_for_player(app_state: Arc<Mutex<AppState>>) {
//     let mut app_state_rc = app_state.lock().unwrap();
//     if let Some(child) = &mut app_state_rc.mpv_child {
//         child.wait().unwrap();
//     }
// }

// pub fn initialize_player(app_state: &mut AppState) {
//     if let Some(child) = &mut app_state.mpv_child {
//         child.kill().unwrap();
//     }
//     app_state.track_queue.next_track();
//     play_track(app_state);
// }

pub fn play_track(app_state: &mut AppState) {
    let tp_opt = app_state.get_curr_track_path();
    let track_path: PathBuf;
    if let Some(tp) = tp_opt {
        track_path = tp
    } else {
        return;
    }

    debug!(
        "Now playing : {}",
        track_path.file_name().unwrap().to_str().unwrap()
    );

    if let Some(child) = &mut app_state.mpv_child {
        child.kill().unwrap();
    }

    app_state.curr_trackinfo = match app_state.get_curr_track_info() {
        Some(t_info_ref) => Some(t_info_ref.clone()),
        None => None
    };

    let v = get_album_cover(&app_state.get_curr_track_path().unwrap());
    app_state.curr_albumcover = Some(Arc::from(v));

    app_state.mpv_child = Some(
        Command::new("mpv")
            .arg(track_path.to_str().unwrap())
            .arg("--no-terminal")
            .arg("--no-audio-display")
            .arg("--audio-samplerate=192000")
            .arg("--audio-format=floatp")
            .arg(format!(
                "--script={}",
                get_luascript_path().to_str().unwrap()
            ))
            .spawn()
            .unwrap(),
    );
    app_state.track_clock = Stopwatch::start_new();

    if let Some(ctx) = &app_state.ctx {
        ctx.request_repaint();
    }
}

pub fn next_track(app_state: &mut AppState) {
    app_state.trackqueue.next_track();
    play_track(app_state);
}

pub fn prev_track(app_state: &mut AppState) {
    app_state.trackqueue.prev_track();
    play_track(app_state);
}

pub async fn player_handler<'a>(app_state: Arc<Mutex<AppState>>, sleep_millis: u64) {
    let ipc_fp = get_ipc_path();
    let mut prev_file_contents = String::new();
    loop {
        let mut app_state_rc = app_state.lock().unwrap();

        // check if current track is over and play next next track if so
        if let Some(child) = &mut app_state_rc.mpv_child {
            if let Some(status) = child.try_wait().unwrap() {
                next_track(&mut app_state_rc);
            }
        }

        let file_contents = read_to_string(&ipc_fp).unwrap();
        if file_contents == prev_file_contents || file_contents.is_empty() {
            drop(app_state_rc);
            sleep(Duration::from_millis(sleep_millis));
            continue;
        } else {
            prev_file_contents = file_contents;
        }

        let mut split_contents = prev_file_contents.split_whitespace();

        app_state_rc.paused = parse_bool(split_contents.next().unwrap());
        if app_state_rc.paused {
            if app_state_rc.track_clock.is_running() {
                app_state_rc.track_clock.stop();
            }
        } else {
            if !app_state_rc.track_clock.is_running() {
                app_state_rc.track_clock.start();
            }
        }

        // check if next track button was pressed
        if parse_bool(split_contents.next().unwrap()) {
            next_track(&mut app_state_rc);
        }

        // check if prev track button was pressed
        if parse_bool(split_contents.next().unwrap()) {
            if app_state_rc.track_clock.elapsed().as_secs() > PREV_SAME_TRACK_TIMEOUT_S {
                play_track(&mut app_state_rc);
            } else {
                prev_track(&mut app_state_rc);
            }
        }

        drop(app_state_rc);
        sleep(Duration::from_millis(sleep_millis));
    }
}
