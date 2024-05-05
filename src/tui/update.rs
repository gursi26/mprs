use ratatui::{prelude::*, widgets::*};
use style::palette::tailwind;
use crate::state::app_state::*;
use rspotify::ClientCredsSpotify;
use crate::tui::user_input::*;

fn update_filter_options(app_state: &mut AppState) {
    let selected_filter_filter = app_state.filter_filter.get_current_selection().unwrap();
    if selected_filter_filter != "All" {
        let mut display_list = app_state
            .track_db
            .track_filter_cache
            .get(&selected_filter_filter)
            .unwrap()
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<String>>();

        if selected_filter_filter == "Playlists" {
            display_list.retain(|x| x != "Liked");
            display_list.push("Liked".to_string());
        } else {
            display_list.retain(|x| x != "None");
            display_list.push("None".to_string());
        }

        app_state.filter.options = display_list;
    } else {
        app_state.filter.options = Vec::new();
    }
}

fn update_tracklist(app_state: &mut AppState) {
    let selected_filter_filter = app_state.filter_filter.get_current_selection().unwrap();

    let selected_tracks = if selected_filter_filter == "All" {
        app_state
            .track_db
            .trackmap
            .keys()
            .map(|x| x.clone())
            .collect()
    } else {
        let selected_filter = app_state.filter.get_current_selection().unwrap();

        app_state
            .track_db
            .track_filter_cache
            .get(&selected_filter_filter)
            .unwrap()
            .get(&selected_filter)
            .unwrap()
            .clone()
    };

    let mut track_list_vec = Vec::new();
    for (i, t_id) in selected_tracks.iter().enumerate() {
        let curr_t_info = app_state.track_db.trackmap.get(t_id).unwrap();
        let mut curr_row = Vec::new();

        curr_row.push(format!("{}", i + 1));

        let mut name = curr_t_info.name.clone();
        curr_row.push(name);

        curr_row.push("".to_string());
        let artists = curr_t_info.artists.clone().unwrap_or(Vec::new());
        curr_row.push(artists.join(", "));
        curr_row.push("".to_string());
        curr_row.push(curr_t_info.album.clone().unwrap_or("".to_string()));
        curr_row.push("".to_string());
        curr_row.push(curr_t_info.playlist.clone());

        let d = curr_t_info.duration;
        let mins = d / 60;
        let duration = format!("{}:{:0>2}", mins, d - (mins * 60));
        curr_row.push(duration);

        let currently_playing_t_id = app_state.get_curr_track_id();

        // let mut row_style = Style::new().bg(match i % 2 {
        //     0 => tailwind::SLATE.c900,
        //     _ => tailwind::SLATE.c950,
        // });

        if let Some(id) = currently_playing_t_id {
            if id == *t_id {
                // row_style = row_style.bg(Color::Green);
                curr_row[0] = " ▶".to_string();
            }
        }

        track_list_vec.push((*t_id, curr_row));
    }
    app_state.track_list.options = track_list_vec;
}

// TODO: Implement separate keybind strings based on which screen is focused and display below
// TODO: <Enter> - Play track, a - add track to curr playlist (only available in track pane when on
// a playlist), a in playlist pane - new playlist with optional spotify link paste to import from
// spotify, l - add track to queue, e - edit currently focused track
pub fn update(app_state: &mut AppState, spotify: Option<&mut ClientCredsSpotify>, force_refresh: bool) {
    if let Some(c) = &mut app_state.search_results.3 {
        if c.try_wait().unwrap().is_some() {
            app_state.search_results.3 = None;
            app_state.search_results.4 = None;
            app_state.search_text_box.1.delete_line_by_end();
            app_state.track_db.add_all_tracks(app_state.search_results.4.clone());
            app_state.display_notification(" Track added ".to_string(), false);
            update(app_state, None, true);
        }
    }

    if let Some(s) = spotify {
        handle_user_input(app_state, s);
    }

    let (c1, c2) = (
        app_state.filter_filter.get_current_selection_idx().unwrap(),
        app_state.filter.get_current_selection_idx().unwrap(),
    );

    if !force_refresh {
        match (
            &mut app_state.prev_filter_filter_selection,
            &mut app_state.prev_filter_selection,
        ) {
            (Some(s1), Some(s2)) => {
                if c1 != *s1 || c2 != *s2 {
                    update_filter_options(app_state);
                    update_tracklist(app_state);
                }
            }
            (_, _) => {
                update_filter_options(app_state);
                update_tracklist(app_state);
            }
        }
    } else {
        update_filter_options(app_state);
        update_tracklist(app_state);
    }
    app_state.prev_filter_filter_selection = Some(c1);
    app_state.prev_filter_selection = Some(c2);
}
