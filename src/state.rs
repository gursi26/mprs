use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;
use stopwatch::Stopwatch;

use crate::track_queue::TrackQueue;

pub struct AppState {
    pub mpv_child: Child,
    pub paused: bool,
    pub track_queue: TrackQueue,
    pub track_clock: Stopwatch
}
