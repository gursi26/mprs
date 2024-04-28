use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;
use stopwatch::Stopwatch;

use crate::track_queue::TrackQueue;

pub struct AppState {
    // internal state attributes
    pub mpv_child: Option<Child>,
    pub paused: bool,
    pub track_queue: TrackQueue,
    pub track_clock: Stopwatch,

    // // ui display attributes
    // // TODO: Replace with db stuff when created (add track_db attribute, make track_list:
    // // Vec<TrackInfo>)
    // pub filter_filter_options: Vec<String>,
    // pub filter_options: Vec<String>,
    // pub track_list: Vec<(String, String, String, String, u64)>,
    // pub curr_track_info: (String, String, String, String, u64),
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            mpv_child: None,
            paused: false,
            track_queue: TrackQueue::new(),
            track_clock: Stopwatch::new()
        }
    }
}
