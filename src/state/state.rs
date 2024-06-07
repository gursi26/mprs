use rspotify::ClientCredsSpotify;
use stopwatch::Stopwatch;

use crate::{
    db::{TrackDB, TrackInfo}, spotdl::{init_spotify_client, SearchResult}, track_queue::TrackQueue
};
use std::{collections::HashMap, path::PathBuf, process::Child, sync::{Arc, Mutex}};

use super::{
    filter_state::F1State, notification_state::NotificationState, tracklist_state::TracklistState,
};

pub struct AppStateWrapper {
    pub app_state: Arc<Mutex<AppState>>,
}

impl Default for AppStateWrapper {
    fn default() -> Self {
        Self {
            app_state: Arc::new(Mutex::new(AppState::default())),
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

    pub curr_trackinfo: Option<TrackInfo>,
    pub curr_albumcover: Option<Arc<[u8]>>,
    pub ctx: Option<eframe::egui::Context>,
    pub new_playlist_name: String,
    pub new_track_search_term: String,
    pub spt_creds: ClientCredsSpotify,
    pub search_results: Option<Vec<SearchResult>>,
    pub selected_result_urls: HashMap<usize, String>,
    pub pending_download_childs: (String, Vec<Child>)
}


pub struct PrevState {
    pub f1_state: F1State,
    pub f2_state: (F1State, String),
    pub trackid: Option<u32>,
    pub shuffle: bool,
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
            trackid: None,
            shuffle: false
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
            curr_trackinfo: None,
            curr_albumcover: None,
            ctx: None,
            new_playlist_name: String::new(),
            new_track_search_term: String::new(),
            spt_creds: init_spotify_client(),
            search_results: None,
            selected_result_urls: HashMap::new(),
            pending_download_childs: (String::new(), Vec::new())
        }
    }
}

impl AppState {
    pub fn get_curr_track_path(&self) -> Option<PathBuf> {
        let curr_trackid = self.trackqueue.get_curr_track();
        if let Some(id) = curr_trackid {
            if let Some(tinfo) = self.trackdb.trackmap.get(&id) {
                Some(tinfo.get_file_path())
            } else {
                None
            }
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
