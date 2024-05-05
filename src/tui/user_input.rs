use crate::consts::*;
use ratatui::{prelude::*, widgets::*};
use crate::{mpv::*, spotdl::*};
use crate::state::app_state::*;
use rspotify::ClientCredsSpotify;
use crate::utils::*;
use tui_textarea::{Key, Input};

pub fn handle_user_input(app_state: &mut AppState, spotify: &mut ClientCredsSpotify) {
    if app_state.search_text_box.0 {
        match crossterm::event::read().unwrap().into() {
            Input { key: Key::Enter, .. } => {
                app_state.search_text_box.0 = false;
                app_state.search_text_box.2 = Some(app_state.search_text_box.1.lines()[0].clone());

                let results = search_tracks(app_state.search_text_box.2.clone().unwrap(), NUM_SEARCH_RESULTS, spotify);
                app_state.search_results.1 = Vec::new();
                app_state.search_results.2 = Vec::new();

                for r in results.into_iter() {
                    app_state.search_results.2.push(r.get_url());

                    let name = r.name;
                    let artists = r.artists.join(", ");
                    let album = r.album;
                    let d_secs = r.duration;
                    let duration = format!("{}:{:0>2}", d_secs / 60, d_secs - ((d_secs / 60) * 60));

                    app_state.search_results.1.push(Row::new(vec![name, artists, album, duration]));
                }
            },
            Input { key: Key::Esc, .. } => {
                app_state.search_text_box.0 = false;
                app_state.focused_window = FocusedWindow::TrackList;
            },
            input => {
                app_state.search_text_box.1.input(input);
            }
        }
        return;
    }
    match get_input_key() {
        UserInput::Quit => app_state.should_quit = true,
        UserInput::FocusLower => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                app_state.focused_window = FocusedWindow::FilterOptions
            }
            _ => {}
        },
        UserInput::FocusUpper => match app_state.focused_window {
            FocusedWindow::FilterOptions => {
                app_state.focused_window = FocusedWindow::FilterFilterOptions
            }
            _ => {}
        },
        UserInput::FocusLeft => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.focused_window = FocusedWindow::FilterFilterOptions
            }
            _ => {}
        },
        UserInput::FocusRight => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                app_state.focused_window = FocusedWindow::TrackList
            }
            FocusedWindow::FilterOptions => app_state.focused_window = FocusedWindow::TrackList,
            _ => {}
        },
        UserInput::SelectLower => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.scroll_down_single();
                app_state.filter.set_scroll(0);
                app_state.track_list.set_scroll(0);
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.scroll_down_single();
                app_state.track_list.set_scroll(0);
            }
            FocusedWindow::TrackList => {
                app_state.track_list.scroll_down_single();
            },
            FocusedWindow::SearchPopup => {
                let s_idx = app_state.search_results.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() + 1).min(app_state.search_results.1.len() - 1));
            }
        },
        UserInput::SelectUpper => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.scroll_up_single();
                app_state.filter.set_scroll(0);
                app_state.track_list.set_scroll(0);
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.scroll_up_single();
                app_state.track_list.set_scroll(0);
            }
            FocusedWindow::TrackList => {
                app_state.track_list.scroll_up_single();
            },
            FocusedWindow::SearchPopup => {
                let s_idx = app_state.search_results.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i32 - 1).max(0) as usize);
            }
        },
        UserInput::Select => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_queue.empty_queue();
                let selected_t_id = app_state.track_list.get_current_selection().unwrap().0;
                for t_info in app_state.track_list.options.iter() {
                    let t_id = t_info.0;
                    if t_id != selected_t_id {
                        app_state.track_queue.add_to_reg_queue(t_id.clone());
                    }
                }
                if app_state.shuffle {
                    app_state.track_queue.shuffle_reg_queue();
                }
                app_state
                    .track_queue
                    .prepend_to_reg_queue(selected_t_id.clone());
                initialize_player(app_state);
            }
            FocusedWindow::SearchPopup => {
                let selected_url = app_state.search_results.2.get(app_state.search_results.0.selected().unwrap()).unwrap();
                app_state.search_results.1 = Vec::new();

                app_state.search_results.3 = Some(download_track(selected_url));
                let curr_playlist = app_state.filter.get_current_selection().unwrap();
                app_state.display_notification(format!(" Downloading track to playlist \'{}\' ", &curr_playlist), true);
                app_state.search_results.4 = Some(curr_playlist.clone());

                app_state.search_text_box.0 = false;
                app_state.search_text_box.2 = None;
                app_state.focused_window = FocusedWindow::TrackList;
            }
            _ => {
                app_state.focused_window = FocusedWindow::TrackList;
            }
        },
        UserInput::JumpToBottom => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_list.set_scroll_last();
            }
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.set_scroll_last();
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.set_scroll_last();
            },
            _ => {}
        },
        UserInput::JumpToTop => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_list.set_scroll(0);
            }
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.set_scroll(0);
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.set_scroll(0);
            },
            _ => {}
        },
        UserInput::JumpMultipleUp => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_list.scroll(-MULTIPLE_JUMP_DISTANCE);
            }
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.scroll(-MULTIPLE_JUMP_DISTANCE);
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.scroll(-MULTIPLE_JUMP_DISTANCE);
            },
            _ => {}
        },
        UserInput::JumpMultipleDown => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_list.scroll(MULTIPLE_JUMP_DISTANCE);
            }
            FocusedWindow::FilterFilterOptions => {
                app_state.filter_filter.scroll(MULTIPLE_JUMP_DISTANCE);
            }
            FocusedWindow::FilterOptions => {
                app_state.filter.scroll(MULTIPLE_JUMP_DISTANCE);
            }
            _ => {}
        },
        UserInput::Delete => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let s_id = app_state.track_list.get_current_selection().unwrap().0;
                app_state.display_deletion_window = Some(DeleteType::TrackDelete(s_id));
            }
            FocusedWindow::FilterOptions => {
                let ff = app_state.filter_filter.get_current_selection().unwrap();
                if ff == "Playlists" {
                    let pname = app_state.filter.get_current_selection().unwrap();
                    if pname != "Liked" {
                        app_state.display_deletion_window =
                            Some(DeleteType::PlaylistDelete(pname.clone()));
                    }
                }
            }
            _ => {}
        },
        UserInput::ConfirmYes => {
            if let Some(_) = app_state.display_deletion_window {
                app_state.confirmed = Some(true);
            }
        }
        UserInput::ConfirmNo => {
            if let Some(_) = app_state.display_deletion_window {
                app_state.confirmed = Some(false);
            } else {
                if let FocusedWindow::TrackList = app_state.focused_window {
                    let curr_track_id = app_state.track_list.get_current_selection().unwrap().0;
                    let track_name = app_state
                        .track_db
                        .trackmap
                        .get(&curr_track_id)
                        .unwrap()
                        .name
                        .clone();
                    app_state.track_queue.play_next(curr_track_id);
                    app_state
                        .display_notification(format!(" Playing track \'{}\' next ", track_name), false);
                }
            }
        }
        UserInput::ToggleShuffle => {
            app_state.shuffle = !app_state.shuffle;
            app_state.track_queue.shuffle_reg_queue();
        }
        UserInput::AddToQueue => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let curr_track_id = app_state.track_list.get_current_selection().unwrap().0;
                let track_name = app_state
                    .track_db
                    .trackmap
                    .get(&curr_track_id)
                    .unwrap()
                    .name
                    .clone();
                app_state.track_queue.add_to_queue(curr_track_id);
                app_state
                    .display_notification(format!(" Added track \'{}\' to queue ", track_name), false);
            }
            _ => {}
        },
        UserInput::AddTrackOrPlaylist => {
            if app_state.filter_filter.get_current_selection().unwrap() != "Playlists" {
                return;
            }
            match app_state.focused_window {
                FocusedWindow::TrackList => {
                    app_state.search_text_box.0 = true;
                    app_state.focused_window = FocusedWindow::SearchPopup;
                },
                FocusedWindow::FilterOptions => {

                }
                _ => {}
            }
        }
        UserInput::Escape => match app_state.focused_window {
            FocusedWindow::SearchPopup => {
                app_state.focused_window = FocusedWindow::TrackList;
                app_state.search_text_box.0 = false;
                app_state.search_text_box.2 = None;
            },
            _ => {}
        }
        _ => {}
    }
}
