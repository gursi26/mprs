use crate::db::TrackDB;

use super::{
    filter_state::F1State, notification_state::NotificationState, tracklist_state::TracklistState,
};

pub struct AppState {
    pub shuffle: bool,
    pub notification: NotificationState,
    pub tracklist_state: TracklistState,
    pub f1_state: F1State,
    pub f2_state: String,

    pub trackdb: TrackDB,

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
            shuffle: false,
            notification: NotificationState::default(),
            tracklist_state: TracklistState::default(),
            f1_state: F1State::Playlists,
            f2_state: default_playlist.clone(),
            trackdb: tdb,

            prev_state
        }
    }
}
