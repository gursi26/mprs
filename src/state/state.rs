use stopwatch::Stopwatch;

use crate::{
    db::{TrackDB, TrackInfo},
    track_queue::TrackQueue,
};
use std::{path::PathBuf, process::Child, sync::{Arc, Mutex}};

use super::{
    filter_state::F1State, notification_state::NotificationState, tracklist_state::TracklistState,
};

pub struct AppStateWrapper {
    pub app_state: Arc<Mutex<AppState>>
}

impl Default for AppStateWrapper {
    fn default() -> Self {
        Self {
            app_state: Arc::new(Mutex::new(AppState::default()))
        }
    }
}

pub struct AppState {
    pub paused: bool,
    pub mpv_child: Option<Child>,

    pub shuffle: bool,
    pub notification: NotificationState,
    pub tracklist_state: TracklistState,
    pub f1_state: F1State,
    pub f2_state: String,

    pub trackdb: TrackDB,
    pub trackqueue: TrackQueue,
    pub track_clock: Stopwatch,
    pub prev_state: PrevState,
}


pub struct PrevState {
    pub f1_state: F1State,
    pub f2_state: (F1State, String),
}

impl Default for AppState {
    fn default() -> Self {
        let tdb = TrackDB::init();
        let default_playlist = *tdb
            .track_filter_cache
            .get(&F1State::Playlists)
            .unwrap()
            .keys()
            .collect::<Vec<&String>>()
            .get(0)
            .unwrap();

        let prev_state = PrevState {
            f1_state: F1State::Playlists,
            f2_state: (F1State::All, "All".to_string()),
        };

        Self {
            paused: true,
            mpv_child: None,
            shuffle: false,
            notification: NotificationState::default(),
            tracklist_state: TracklistState::default(),
            f1_state: F1State::Playlists,
            f2_state: default_playlist.clone(),

            trackdb: tdb,
            trackqueue: TrackQueue::new(),
            track_clock: Stopwatch::new(),
            prev_state,
        }
    }
}

impl AppState {
    pub fn get_curr_track_path(&self) -> Option<PathBuf> {
        let curr_trackid = self.trackqueue.get_curr_track();
        if let Some(id) = curr_trackid {
            let tinfo = self.trackdb.trackmap.get(&id).unwrap();
            Some(tinfo.get_file_path())
        } else {
            None
        }
    }

    pub fn get_curr_track_info(&self) -> Option<&TrackInfo> {
        let curr_trackid = self.trackqueue.get_curr_track();
        if let Some(id) = curr_trackid {
            let tinfo = self.trackdb.trackmap.get(&id).unwrap();
            Some(tinfo)
        } else {
            None
        }
    }

    pub fn add_curr_tracklist_to_regular_queue(&mut self) {
        let curr_track_ids = self
            .trackdb
            .track_filter_cache
            .get(&self.f1_state)
            .unwrap()
            .get(&self.f2_state)
            .unwrap();

        for tid in curr_track_ids.iter() {
            self.trackqueue.add_to_reg_queue(tid.clone());
        }
    }
}
