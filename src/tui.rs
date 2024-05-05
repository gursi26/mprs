use anyhow::Result;
use crossterm::event::DisableMouseCapture;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use image::codecs::png::FilterType;
use ratatui::prelude::{CrosstermBackend, Frame, Terminal};
use ratatui::{prelude::*, widgets::*};
use ratatui_image::{Resize, StatefulImage};
use rspotify::ClientCredsSpotify;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use stopwatch::Stopwatch;
use style::palette::tailwind;
use tui_textarea::{Input, Key, TextArea};

use crate::spotdl::{download_track, search_tracks};
use crate::NUM_SEARCH_RESULTS;
use crate::{
    mpv::{initialize_player, play_track},
    state::{AppState, DeleteType, FocusedWindow},
    track_queue::TrackType,
    utils::{
        centered_rect, get_album_cover, get_input_key, get_keybind_string,
        get_progress_display_str, wrap_string, UserInput,
    },
    MULTIPLE_JUMP_DISTANCE, NOTIFICATION_TIMEOUT_S, UI_SLEEP_DURATION_MS,
};

const UNSELECTED_COLOR: Color = Color::White;
const SELECT_COLOR: Color = Color::Green;

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c900;
const GAUGE_BG_COLOR: Color = tailwind::BLUE.c900;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c950;

// inner_layout split
const LEFT_SIDEBAR_SIZE: u16 = 22;
const RIGHT_TRACKLIST_SIZE: u16 = 100 - LEFT_SIDEBAR_SIZE;

// left sidebar split
const FILTER_FILTER_OPTIONS_SIZE: u16 = 11;
const FILTER_OPTIONS_SIZE: u16 = 39;
const CURR_TRACK_INFO_SIZE: u16 = 50;

// curr track info split
const ALBUM_COVER_SIZE: u16 = 80;
const GAUGE_SIZE: u16 = 3;
const TEXT_SIZE: u16 = 100 - (ALBUM_COVER_SIZE + GAUGE_SIZE);

pub fn run<'a>(app_state: Arc<Mutex<AppState<'a>>>, spotify: &mut ClientCredsSpotify) -> Result<()> {
    enable_raw_mode()?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut t = Terminal::new(CrosstermBackend::new(stdout))?;

    loop {
        let mut curr_app_state_rc = app_state.lock().unwrap();

        update(&mut curr_app_state_rc, Some(spotify), false);

        t.draw(|f| {
            ui(&mut curr_app_state_rc, f);
        })?;

        if curr_app_state_rc.should_quit {
            break;
        }
        drop(curr_app_state_rc);
        sleep(Duration::from_millis(UI_SLEEP_DURATION_MS));
    }

    shutdown().unwrap();
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn handle_user_input(app_state: &mut AppState, spotify: &mut ClientCredsSpotify) {
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
                let s_idx = app_state.filter_filter_options.0.selected_mut();
                *s_idx =
                    Some((s_idx.unwrap() + 1).min(app_state.filter_filter_options.1.len() - 1));

                let s2_idx = app_state.filter_options.0.selected_mut();
                *s2_idx = Some(0);

                let s3_idx = app_state.display_track_list.0.selected_mut();
                *s3_idx = Some(0);
            }
            FocusedWindow::FilterOptions => {
                let s_idx = app_state.filter_options.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() + 1).min(app_state.filter_options.1.len() - 1));

                let s3_idx = app_state.display_track_list.0.selected_mut();
                *s3_idx = Some(0);
            }
            FocusedWindow::TrackList => {
                let s_idx = app_state.display_track_list.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() + 1).min(app_state.display_track_list.1.len() - 1));
            },
            FocusedWindow::SearchPopup => {
                let s_idx = app_state.search_results.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() + 1).min(app_state.search_results.1.len() - 1));
            }
        },
        UserInput::SelectUpper => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                let s_idx = app_state.filter_filter_options.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i32 - 1).max(0) as usize);

                let s2_idx = app_state.filter_options.0.selected_mut();
                *s2_idx = Some(0);

                let s3_idx = app_state.display_track_list.0.selected_mut();
                *s3_idx = Some(0);
            }
            FocusedWindow::FilterOptions => {
                let s_idx = app_state.filter_options.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i32 - 1).max(0) as usize);

                let s3_idx = app_state.display_track_list.0.selected_mut();
                *s3_idx = Some(0);
            }
            FocusedWindow::TrackList => {
                let s_idx = app_state.display_track_list.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i32 - 1).max(0) as usize);
            },
            FocusedWindow::SearchPopup => {
                let s_idx = app_state.search_results.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i32 - 1).max(0) as usize);
            }
        },
        UserInput::Select => match app_state.focused_window {
            FocusedWindow::TrackList => {
                app_state.track_queue.empty_queue();
                let selected_t_id = app_state
                    .display_track_list
                    .2
                    .get(app_state.display_track_list.0.selected().unwrap())
                    .unwrap();
                for t_id in app_state.display_track_list.2.iter() {
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
                let curr_playlist = app_state.filter_options.1.get(app_state.filter_options.0.selected().unwrap()).unwrap().clone();
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
                let s = app_state.display_track_list.0.selected_mut();
                *s = Some(app_state.display_track_list.1.len() - 1);
            }
            FocusedWindow::FilterFilterOptions => {
                let s = app_state.filter_filter_options.0.selected_mut();
                *s = Some(app_state.filter_filter_options.1.len() - 1);
            }
            FocusedWindow::FilterOptions => {
                let s = app_state.filter_options.0.selected_mut();
                *s = Some(app_state.filter_options.1.len() - 1);
            },
            _ => {}
        },
        UserInput::JumpToTop => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let s = app_state.display_track_list.0.selected_mut();
                *s = Some(0);
            }
            FocusedWindow::FilterFilterOptions => {
                let s = app_state.filter_filter_options.0.selected_mut();
                *s = Some(0);
            }
            FocusedWindow::FilterOptions => {
                let s = app_state.filter_options.0.selected_mut();
                *s = Some(0);
            },
            _ => {}
        },
        UserInput::JumpMultipleUp => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let s = app_state.display_track_list.0.selected_mut();
                match s.clone() {
                    Some(x) => *s = Some((x as i32 - MULTIPLE_JUMP_DISTANCE).max(0) as usize),
                    None => {}
                }
            }
            FocusedWindow::FilterFilterOptions => {
                let s = app_state.filter_filter_options.0.selected_mut();
                match s.clone() {
                    Some(x) => *s = Some((x as i32 - MULTIPLE_JUMP_DISTANCE).max(0) as usize),
                    None => {}
                }
            }
            FocusedWindow::FilterOptions => {
                let s = app_state.filter_options.0.selected_mut();
                match s.clone() {
                    Some(x) => *s = Some((x as i32 - MULTIPLE_JUMP_DISTANCE).max(0) as usize),
                    None => {}
                }
            },
            _ => {}
        },
        UserInput::JumpMultipleDown => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let s = app_state.display_track_list.0.selected_mut();
                match s.clone() {
                    Some(x) => {
                        *s = Some(
                            (x as i32 + MULTIPLE_JUMP_DISTANCE)
                                .min(app_state.display_track_list.1.len() as i32 - 1)
                                as usize,
                        )
                    }
                    None => {}
                }
            }
            FocusedWindow::FilterFilterOptions => {
                let s = app_state.filter_filter_options.0.selected_mut();
                match s.clone() {
                    Some(x) => {
                        *s = Some(
                            (x as i32 + MULTIPLE_JUMP_DISTANCE)
                                .min(app_state.filter_filter_options.1.len() as i32 - 1)
                                as usize,
                        )
                    }
                    None => {}
                }
            }
            FocusedWindow::FilterOptions => {
                let s = app_state.filter_options.0.selected_mut();
                match s.clone() {
                    Some(x) => {
                        *s = Some(
                            (x as i32 + MULTIPLE_JUMP_DISTANCE)
                                .min(app_state.filter_options.1.len() as i32 - 1)
                                as usize,
                        )
                    }
                    None => {}
                }
            }
            _ => {}
        },
        UserInput::Delete => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let s_id = app_state
                    .display_track_list
                    .2
                    .get(app_state.display_track_list.0.selected().unwrap())
                    .unwrap();
                app_state.display_deletion_window = Some(DeleteType::TrackDelete(*s_id));
            }
            FocusedWindow::FilterOptions => {
                let ff = app_state.filter_filter_options.1
                    [app_state.filter_filter_options.0.selected().unwrap()];
                if ff == "Playlists" {
                    let pname = app_state
                        .filter_options
                        .1
                        .get(app_state.filter_options.0.selected().unwrap())
                        .unwrap();
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
                    let curr_track_id = app_state
                        .display_track_list
                        .2
                        .get(app_state.display_track_list.0.selected().unwrap())
                        .unwrap();
                    let track_name = app_state
                        .track_db
                        .trackmap
                        .get(curr_track_id)
                        .unwrap()
                        .name
                        .clone();
                    app_state.track_queue.play_next(*curr_track_id);
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
                let curr_track_id = app_state
                    .display_track_list
                    .2
                    .get(app_state.display_track_list.0.selected().unwrap())
                    .unwrap();
                let track_name = app_state
                    .track_db
                    .trackmap
                    .get(curr_track_id)
                    .unwrap()
                    .name
                    .clone();
                app_state.track_queue.add_to_queue(*curr_track_id);
                app_state
                    .display_notification(format!(" Added track \'{}\' to queue ", track_name), false);
            }
            _ => {}
        },
        UserInput::AddTrackOrPlaylist => {
            if app_state.filter_filter_options.1[app_state.filter_filter_options.0.selected().unwrap()] != "Playlists" {
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

fn update_filter_options(app_state: &mut AppState) {
    let selected_filter_filter =
        app_state.filter_filter_options.1[app_state.filter_filter_options.0.selected().unwrap()];
    if selected_filter_filter != "All" {
        let mut display_list = app_state
            .track_db
            .track_filter_cache
            .get(selected_filter_filter)
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

        app_state.filter_options.1 = display_list;
    } else {
        app_state.filter_options.1 = Vec::new();
    }
}

fn update_tracklist(app_state: &mut AppState) {
    let selected_filter_filter =
        app_state.filter_filter_options.1[app_state.filter_filter_options.0.selected().unwrap()];

    let selected_tracks = if selected_filter_filter == "All" {
        app_state
            .track_db
            .trackmap
            .keys()
            .map(|x| x.clone())
            .collect()
    } else {
        let selected_filter = app_state
            .filter_options
            .1
            .get(app_state.filter_options.0.selected().unwrap())
            .unwrap();

        app_state
            .track_db
            .track_filter_cache
            .get(selected_filter_filter)
            .unwrap()
            .get(selected_filter)
            .unwrap()
            .clone()
    };

    let mut track_list_vec = Vec::new();
    let mut t_id_vec = Vec::new();
    for (i, t_id) in selected_tracks.iter().enumerate() {
        t_id_vec.push(t_id.clone());
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

        let mut row_style = Style::new().bg(match i % 2 {
            0 => tailwind::SLATE.c900,
            _ => tailwind::SLATE.c950,
        });

        if let Some(id) = currently_playing_t_id {
            if id == *t_id {
                row_style = row_style.bg(Color::Green);
                curr_row[0] = " ▶".to_string();
            }
        }

        track_list_vec.push(Row::new(curr_row).height(1).style(row_style));
    }
    app_state.display_track_list.1 = track_list_vec;
    app_state.display_track_list.2 = t_id_vec;
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
        app_state.filter_filter_options.0.selected().unwrap(),
        app_state.filter_options.0.selected().unwrap(),
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

fn render_header_footer(
    app_state: &mut AppState,
    frame: &mut Frame,
    header_space: Rect,
    footer_space: Rect,
) {
    frame.render_widget(
        Block::new().borders(Borders::TOP).title(format!(
            " {} - v{} ",
            std::env!("CARGO_PKG_NAME"),
            std::env!("CARGO_PKG_VERSION"),
        )),
        header_space,
    );

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(get_keybind_string(app_state)),
        footer_space,
    );
    frame.render_widget(
        Block::new()
            .title(format!(" Shuffle: {} ", app_state.shuffle))
            .title_alignment(Alignment::Right),
        footer_space,
    );
}

fn render_filter_filter_options(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let list = List::new(app_state.filter_filter_options.1)
        .block(
            Block::default()
                .title("")
                .borders(Borders::ALL)
                .border_style(Style::new().fg(match app_state.focused_window {
                    FocusedWindow::FilterFilterOptions => SELECT_COLOR,
                    _ => UNSELECTED_COLOR,
                })),
        )
        .highlight_style(Style::new().fg(SELECT_COLOR))
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, space, &mut app_state.filter_filter_options.0);
}

fn render_filter_options(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let curr_selected_filter_filter =
        app_state.filter_filter_options.1[app_state.filter_filter_options.0.selected().unwrap()];

    let filter_list = List::new(app_state.filter_options.1.clone())
        .block(
            Block::default()
                .title(match curr_selected_filter_filter {
                    "All" => "".to_string(),
                    _ => format!(
                        " {} ({}) ",
                        curr_selected_filter_filter,
                        app_state.filter_options.1.len()
                    ),
                })
                .borders(Borders::ALL)
                .border_style(Style::new().fg(match app_state.focused_window {
                    FocusedWindow::FilterOptions => SELECT_COLOR,
                    _ => UNSELECTED_COLOR,
                })),
        )
        .highlight_style(Style::new().fg(SELECT_COLOR))
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(filter_list, space, &mut app_state.filter_options.0);
}

fn render_tracklist(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let widths = [
        // id + name + padding
        Constraint::Percentage(3),
        Constraint::Percentage(35),
        Constraint::Percentage(2),
        // artist + padding
        Constraint::Percentage(23),
        Constraint::Percentage(2),
        // album + padding
        Constraint::Percentage(16),
        Constraint::Percentage(2),
        // playlist + duration
        Constraint::Percentage(10),
        Constraint::Percentage(7),
    ];

    let mut curr_track_list_name = match app_state
        .filter_options
        .1
        .get(app_state.filter_options.0.selected().unwrap())
    {
        Some(x) => x.clone(),
        None => "".to_string(),
    };

    let curr_selected_filter_filter =
        app_state.filter_filter_options.1[app_state.filter_filter_options.0.selected().unwrap()];

    if let "All" = curr_selected_filter_filter {
        curr_track_list_name = "All Tracks".to_string();
    }

    let track_table = Table::new(app_state.display_track_list.1.clone(), widths)
        .block(
            Block::default()
                .title(format!(
                    " {} ({}) ",
                    curr_track_list_name,
                    app_state.display_track_list.1.len()
                ))
                .borders(Borders::ALL)
                .border_style(Style::new().fg(match app_state.focused_window {
                    FocusedWindow::TrackList => SELECT_COLOR,
                    _ => UNSELECTED_COLOR,
                })),
        )
        .header(
            Row::new(vec![
                "", "Title", "", "Artist", "", "Album", "", "Playlist", "Duration",
            ])
            .style(
                Style::new()
                    .fg(tailwind::SLATE.c200)
                    .bg(tailwind::BLUE.c900)
                    .add_modifier(Modifier::BOLD),
            )
            .height(1),
        )
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().bg(tailwind::BLUE.c400).fg(Color::Black));

    frame.render_stateful_widget(track_table, space, &mut app_state.display_track_list.0);
}

fn render_left_sidebar(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let left_sidebar_block = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(FILTER_FILTER_OPTIONS_SIZE),
            Constraint::Percentage(FILTER_OPTIONS_SIZE),
            Constraint::Percentage(CURR_TRACK_INFO_SIZE),
        ],
    )
    .split(space);

    render_filter_filter_options(frame, app_state, left_sidebar_block[0]);
    render_filter_options(frame, app_state, left_sidebar_block[1]);
    render_curr_track_info(frame, app_state, left_sidebar_block[2]);
}

// TODO: Figure out how to center album cover
fn render_album_cover(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let img_widget = StatefulImage::new(None);
    let img_state = match &mut app_state.curr_track_cover {
        Some(state) => state,
        None => return,
    };

    let cover_block = Block::new().borders(Borders::ALL).inner(space);

    frame.render_stateful_widget(img_widget, cover_block, img_state)
}

fn render_progress_gauge(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let secs_played = app_state.track_clock.elapsed().as_secs_f64();
    let total_secs = match &app_state.curr_track_info {
        Some(t_info) => t_info.duration as f64,
        None => 0.0,
    };

    let percent = if total_secs == 0.0 {
        0.0
    } else {
        secs_played / total_secs
    }
    .clamp(0.0, 1.0);

    let d_str = get_progress_display_str(secs_played, total_secs);
    let gauge = Gauge::default()
        .ratio(percent)
        .label(d_str)
        .gauge_style(Style::new().bg(GAUGE_BG_COLOR))
        .use_unicode(true)
        .block(Block::new().padding(Padding {
            left: 2,
            right: 2,
            top: 0,
            bottom: 0,
        }));

    frame.render_widget(gauge, space);
}

fn render_curr_track_text(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let track_name = match &app_state.curr_track_info {
        Some(t_info) => t_info.name.clone(),
        None => "None".to_string(),
    };

    let track_artists = match &app_state.curr_track_info {
        Some(t_info) => match &t_info.artists {
            Some(ar) => ar.join(", "),
            None => "None".to_string(),
        },
        None => "None".to_string(),
    };

    let track_album = match &app_state.curr_track_info {
        Some(t_info) => match &t_info.album {
            Some(al) => al.clone(),
            None => "None".to_string(),
        },
        None => "None".to_string(),
    };

    frame.render_widget(
        Paragraph::new(format!(
            "{}\n{}\n{}",
            track_name, track_artists, track_album
        ))
        .centered()
        .block(Block::new().padding(Padding {
            left: 4,
            right: 4,
            top: 0,
            bottom: 0,
        })),
        space,
    );
}

fn render_curr_track_info(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let curr_track_block = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(ALBUM_COVER_SIZE),
            Constraint::Percentage(GAUGE_SIZE),
            Constraint::Percentage(TEXT_SIZE),
            Constraint::Percentage(2),
        ],
    )
    .split(space);

    frame.render_widget(Block::new().borders(Borders::ALL), space);

    render_album_cover(frame, app_state, curr_track_block[0]);
    render_progress_gauge(frame, app_state, curr_track_block[1]);
    render_curr_track_text(frame, app_state, curr_track_block[2]);
}

fn render_confirmation_window(app_state: &mut AppState, frame: &mut Frame) {
    if let Some(delete_enum) = &app_state.display_deletion_window {
        let centered_rect = centered_rect(30, 10, frame.size());
        frame.render_widget(ratatui::widgets::Clear, centered_rect);
        frame.render_widget(Block::new().borders(Borders::ALL), centered_rect);

        let message;
        match delete_enum {
            DeleteType::TrackDelete(id) => {
                let track_name = app_state.track_db.trackmap.get(&id).unwrap().name.clone();
                message = format!("Confirm delete track \'{}\'?", track_name);
            }
            DeleteType::PlaylistDelete(name) => {
                message = format!("Confirm delete playlist \'{}\'?", name)
            }
        }

        let p_areas = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        )
        .split(centered_rect);
        frame.render_widget(
            Paragraph::new(message).wrap(Wrap { trim: true }),
            p_areas[0].inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
        );
        frame.render_widget(Paragraph::new("[y] Yes \t [n] No").centered(), p_areas[1]);

        match app_state.confirmed {
            Some(x) => {
                if x {
                    match delete_enum {
                        DeleteType::TrackDelete(idx) => {
                            app_state.track_db.remove_track(*idx, None);
                        }
                        DeleteType::PlaylistDelete(name) => {
                            app_state.track_db.remove_playlist(name.clone());
                        }
                    }
                    app_state.display_deletion_window = None;
                    app_state.confirmed = None;
                    update(app_state, None, true);
                } else {
                    app_state.display_deletion_window = None;
                    app_state.confirmed = None;
                }
            }
            None => {}
        }
    }
}

fn render_notification(app_state: &mut AppState, frame: &mut Frame, space: Rect) {
    frame.render_widget(
        Block::new()
            .title(app_state.notification.0.clone())
            .title_alignment(Alignment::Right),
        space,
    );

    if app_state.notification.1.is_running() {
        if app_state.notification.1.elapsed().as_secs() > NOTIFICATION_TIMEOUT_S && !app_state.notification.2 {
            app_state.notification.0 = "".to_string();
            app_state.notification.1 = Stopwatch::new();
        }
    }
}

fn render_search_popup(app_state: &mut AppState, frame: &mut Frame) {
    if app_state.search_text_box.0 || app_state.search_text_box.2.is_some() {
        let centered_rect = centered_rect(50, 33, frame.size());
        frame.render_widget(ratatui::widgets::Clear, centered_rect);
        frame.render_widget(Block::new().borders(Borders::ALL), centered_rect);

        let search_split = Layout::new(
            Direction::Vertical,
            [Constraint::Length(5), Constraint::Min(0)],
        )
        .split(centered_rect);
        frame.render_widget(
            app_state.search_text_box.1.widget(),
            search_split[0].inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
        );

        if app_state.search_results.3.is_some() {
            frame.render_widget(Paragraph::new("Downloading track..."), search_split[1].inner(&Margin { horizontal: 1, vertical: 1 }));
            return;
        }

        let widths = [
            Constraint::Percentage(50),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ];
        let table = Table::new(app_state.search_results.1.clone(), widths)
            .header(Row::new(vec!["Title", "Artists", "Album", "Duration"]))
            .highlight_style(Style::new().fg(SELECT_COLOR));

        frame.render_stateful_widget(table, search_split[1].inner(&Margin { horizontal: 1, vertical: 0 }), &mut app_state.search_results.0);
    }
}

fn ui(app_state: &mut AppState, frame: &mut Frame) {
    // header and footer split
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    // inner layout split
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(LEFT_SIDEBAR_SIZE),
            Constraint::Percentage(RIGHT_TRACKLIST_SIZE),
        ],
    )
    .split(main_layout[1]);

    render_header_footer(app_state, frame, main_layout[0], main_layout[2]);
    render_notification(app_state, frame, main_layout[0]);
    render_tracklist(frame, app_state, inner_layout[1]);
    render_left_sidebar(frame, app_state, inner_layout[0]);
    render_confirmation_window(app_state, frame);
    render_search_popup(app_state, frame);
}
