use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;

use crate::track_queue::TrackQueue;

pub struct AppState {
    pub mpv_child: Child,
    pub paused: bool,
    pub next_pressed: bool,
    pub prev_pressed: bool,
    pub track_queue: TrackQueue,
}
