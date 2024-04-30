use std::path::PathBuf;
use std::sync::Arc;
use std::process::Child;
use std::sync::Mutex;
use stopwatch::Stopwatch;

use crate::db::TrackDB;
use crate::track_queue::TrackQueue;

pub struct AppState {
    // internal state attributes
    pub mpv_child: Option<Child>,
    pub paused: bool,
    pub track_queue: TrackQueue,
    pub track_clock: Stopwatch,
    pub track_db: TrackDB

    // ui display attributes
    // TODO: Replace with db stuff when created (add track_db attribute, make track_list:
    // Vec<TrackInfo>)
    // pub filter_filter_options: Vec<String>,
    // pub filter_options: Vec<String>,
    // pub track_list: Vec<(String, String, String, String, u64)>,
    // pub curr_track_info: (String, String, String, String, u64),
}

impl AppState {
    pub fn get_curr_track_path(&self) -> Option<PathBuf> {
        let curr_track_id = self.track_queue.get_curr_track();
        if let Some(id) = curr_track_id {
            let t_info = self.track_db.trackmap.get(&id).unwrap();
            Some(t_info.get_file_path())
        } else {
            None
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            mpv_child: None,
            paused: false,
            track_queue: TrackQueue::new(),
            track_clock: Stopwatch::new(),
            track_db: TrackDB::init()
        }
    }
}
