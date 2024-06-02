use stopwatch::Stopwatch;

use crate::{db::{TrackDB, TrackInfo}, track_queue::TrackQueue};
use std::{path::PathBuf, process::Child};

use super::{
    filter_state::F1State, notification_state::NotificationState, tracklist_state::TracklistState,
};

pub struct AppState {
    pub mpv_child: Option<Child>,

    pub shuffle: bool,
    pub notification: NotificationState,
    pub tracklist_state: TracklistState,
    pub f1_state: F1State,
    pub f2_state: String,

    pub trackdb: TrackDB,
    pub trackqueue: TrackQueue,
    pub track_clock: Stopwatch,
    pub prev_state: PrevState
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
            f2_state: (F1State::All, "All".to_string())
        };

        Self {
            mpv_child: None,
            shuffle: false,
            notification: NotificationState::default(),
            tracklist_state: TracklistState::default(),
            f1_state: F1State::Playlists,
            f2_state: default_playlist.clone(),

            trackdb: tdb,
            trackqueue: TrackQueue::new(),
            track_clock: Stopwatch::new(),
            prev_state
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
}
