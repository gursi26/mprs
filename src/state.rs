use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;

use crate::track_queue::TrackQueue;

pub struct AppState {
    pub mpv_child: Arc<Mutex<Child>>,
    pub paused: Arc<Mutex<bool>>,
    pub next_pressed: Arc<Mutex<bool>>,
    pub prev_pressed: Arc<Mutex<bool>>,
    pub track_queue: Arc<Mutex<TrackQueue>>,
}
