use ratatui::widgets::ListState;
use ratatui::widgets::Row;
use ratatui::widgets::TableState;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use stopwatch::Stopwatch;

use crate::db::{TrackDB, TrackInfo};
use crate::track_queue::TrackQueue;
use crate::track_queue::TrackType;
use crate::utils::get_album_cover;

pub enum FocusedWindow {
    FilterFilterOptions,
    FilterOptions,
    TrackList,
}

pub enum DeleteType {
    TrackDelete(u32),
    PlaylistDelete(String)
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
    pub display_track_list: (TableState, Vec<Row<'a>>, Vec<u32>),
    pub curr_track_info: Option<TrackInfo>,
    pub curr_track_cover: Option<Box<dyn StatefulProtocol>>,
    pub focused_window: FocusedWindow,
    pub display_deletion_window: Option<DeleteType>,
    pub confirmed: Option<bool>,
    pub shuffle: bool,
    pub notification: (String, Stopwatch),

    // differencing attributes
    pub prev_filter_filter_selection: Option<usize>,
    pub prev_filter_selection: Option<usize>,
}

impl<'a> AppState<'a> {
    pub fn display_notification(&mut self, message: String) {
        self.notification.0 = message;
        self.notification.1.start();
    }

    pub fn get_curr_track_path(&self) -> Option<PathBuf> {
        let curr_track_id = self.track_queue.get_curr_track();
        if let Some(id) = curr_track_id {
            let t_info = self.track_db.trackmap.get(&id).unwrap();
            Some(t_info.get_file_path())
        } else {
            None
        }
    }

    pub fn get_curr_track_id(&self) -> Option<u32> {
        match self.track_queue.curr_track {
            TrackType::RegQueueTrack(id) => Some(id),
            TrackType::ExQueueTrack(id) => Some(id),
            TrackType::None => None,
        }
    }

    pub fn get_curr_track_info(&self) -> Option<&TrackInfo> {
        match self.get_curr_track_id() {
            Some(id) => Some(self.track_db.trackmap.get(&id).unwrap()),
            None => None
        }
    }

    pub fn update_curr_album_cover(&mut self) {
        let img_bytes = get_album_cover(&match &self.curr_track_info {
            Some(c) => c.get_file_path(),
            None => return
        });
        let img = image::load_from_memory(&img_bytes).unwrap();

        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

        self.curr_track_cover = Some(picker.new_resize_protocol(img));
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
            display_deletion_window: None,
            confirmed: None,
            shuffle: false,
            notification: ("".to_string(), Stopwatch::new()),

            filter_filter_options: (
                ListState::default().with_selected(Some(0)),
                ["Playlists", "Artists", "Albums", "All"],
            ),
            filter_options: (ListState::default().with_selected(Some(0)), Vec::new()),
            display_track_list: (TableState::default().with_selected(Some(0)), Vec::new(), Vec::new()),
            curr_track_info: None,
            curr_track_cover: None,
            focused_window: FocusedWindow::FilterFilterOptions,
            prev_filter_filter_selection: None,
            prev_filter_selection: None
        }
    }
}
