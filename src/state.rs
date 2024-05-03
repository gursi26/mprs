use ratatui::widgets::ListState;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use stopwatch::Stopwatch;

use crate::db::{TrackDB, TrackInfo};
use crate::track_queue::TrackQueue;

pub enum FocusedWindow {
    FilterFilterOptions,
    FilterOptions,
    TrackList,
}

pub struct AppState<'a> {
    // internal state attributes
    pub mpv_child: Option<Child>,
    pub paused: bool,
    pub track_queue: TrackQueue,
    pub track_clock: Stopwatch,
    pub track_db: TrackDB,
    pub should_quit: bool,

    // ui display attributes
    pub filter_filter_options: (ListState, [&'a str; 4]),
    pub filter_options: (ListState, Vec<String>),
    pub display_track_list: Vec<&'a TrackInfo>,
    pub curr_track_info: Option<&'a TrackInfo>,
    pub focused_window: FocusedWindow, // stateful attributes
}

impl<'a> AppState<'a> {
    pub fn get_curr_track_path(&self) -> Option<PathBuf> {
        let curr_track_id = self.track_queue.get_curr_track();
        if let Some(id) = curr_track_id {
            let t_info = self.track_db.trackmap.get(&id).unwrap();
            Some(t_info.get_file_path())
        } else {
            None
        }
    }

    pub fn next_track(&'a mut self) {
        self.track_queue.next_track();
        let t_id = match self.track_queue.get_curr_track() {
            Some(id) => id,
            None => {
                self.curr_track_info = None;
                return;
            }
        };
        let t_info = self.track_db.trackmap.get(&t_id).unwrap();
        self.curr_track_info = Some(t_info);
    }

    pub fn add_playlist_to_queue(&mut self, playlist_name: String) {
        let t_ids = self
            .track_db
            .track_filter_cache
            .get("Playlists")
            .unwrap()
            .get(&playlist_name)
            .unwrap();

        self.track_queue.empty_queue();
        for t_id in t_ids.iter() {
            self.track_queue.add_to_reg_queue(t_id.clone());
        }
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        AppState {
            mpv_child: None,
            paused: false,
            track_queue: TrackQueue::new(),
            track_clock: Stopwatch::new(),
            track_db: TrackDB::init(),
            should_quit: false,

            filter_filter_options: (
                ListState::default().with_selected(Some(0)),
                ["All", "Playlists", "Artists", "Albums"],
            ),
            filter_options: (ListState::default().with_selected(Some(0)), Vec::new()),
            display_track_list: Vec::new(),
            curr_track_info: None,
            focused_window: FocusedWindow::FilterFilterOptions,
        }
    }
}
