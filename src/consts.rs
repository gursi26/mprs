use ratatui::prelude::*;
use style::palette::tailwind;

pub const UNSELECTED_COLOR: Color = Color::White;
pub const SELECT_COLOR: Color = Color::Green;
 
pub const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c900;
pub const GAUGE_BG_COLOR: Color = tailwind::BLUE.c900;
pub const ALT_ROW_COLOR: Color = tailwind::SLATE.c950;
 
 // inner_layout split
pub const LEFT_SIDEBAR_SIZE: u16 = 22;
pub const RIGHT_TRACKLIST_SIZE: u16 = 100 - LEFT_SIDEBAR_SIZE;
 
 // left sidebar split
pub const FILTER_FILTER_OPTIONS_SIZE: u16 = 11;
pub const FILTER_OPTIONS_SIZE: u16 = 39;
pub const CURR_TRACK_INFO_SIZE: u16 = 50;
 
 // curr track info split
pub const ALBUM_COVER_SIZE: u16 = 80;
pub const GAUGE_SIZE: u16 = 3;
pub const TEXT_SIZE: u16 = 100 - (ALBUM_COVER_SIZE + GAUGE_SIZE);
 
pub const MUSIC_DIR: &str = "mprs-tracks";
pub const MPV_STATUS_IPC_FILENAME: &str = ".mpv_status.txt";
pub const MPV_LUASCRIPT_FILENAME: &str = "status_update.lua";
 
pub const PLAYER_HANDLER_TIMEOUT_MS: u64 = 20;
pub const UI_SLEEP_DURATION_MS: u64 = 10;
pub const PREV_SAME_TRACK_TIMEOUT_S: u64 = 3;
pub const KEY_INPUT_POLL_TIMEOUT_MS: u64 = 250;
pub const NOTIFICATION_TIMEOUT_S: u64 = 3;
 
pub const NUM_SEARCH_RESULTS: u32 = 10;
pub const MULTIPLE_JUMP_DISTANCE: i32 = 20;
