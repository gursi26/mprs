use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Frame, Terminal};
use ratatui::{prelude::*, widgets::*};
use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};
use style::palette::tailwind;

use crate::{
    mpv::{initialize_player, play_track}, state::{AppState, FocusedWindow}, utils::{get_input_key, wrap_string, UserInput}, UI_SLEEP_DURATION
};

const UNSELECTED_COLOR: Color = Color::White;
const SELECT_COLOR: Color = Color::Green;

pub async fn run<'a>(app_state: Arc<Mutex<AppState<'a>>>) -> Result<()> {
    startup().unwrap();

    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        let mut curr_app_state_rc = app_state.lock().unwrap();

        update(&mut curr_app_state_rc);

        t.draw(|f| {
            ui(&mut curr_app_state_rc, f);
        })?;

        if curr_app_state_rc.should_quit {
            break;
        }
        drop(curr_app_state_rc);
        sleep(Duration::from_millis(UI_SLEEP_DURATION));
    }

    shutdown().unwrap();
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn handle_user_input(app_state: &mut AppState) {
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
            }
        },
        UserInput::Select => match app_state.focused_window {
            FocusedWindow::TrackList => {
                let selected_t_id = app_state.display_track_list.2.get(app_state.display_track_list.0.selected().unwrap()).unwrap();
                app_state.track_queue.add_to_reg_queue(selected_t_id.clone());
                for t_id in app_state.display_track_list.2.iter() {
                    if t_id != selected_t_id {
                        app_state.track_queue.add_to_reg_queue(t_id.clone());
                    }
                }
                initialize_player(app_state)
            },
            _ => {
                app_state.focused_window = FocusedWindow::TrackList;
            },
        }
        _ => {}
    }
}

// TODO: Implement separate keybind strings based on which screen is focused and display below
// TODO: Split into sub functions
// TODO: Implement prev_state store so that updates only happen when something is changed
// TODO: Implement table view for tracks and scrolling
// TODO: <Enter> - Play track, a - add track to curr playlist (only available in track pane when on
// a playlist), a in playlist pane - new playlist with optional spotify link paste to import from
// spotify, l - add track to queue, e - edit currently focused track
fn update(app_state: &mut AppState) {
    handle_user_input(app_state);

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
        track_list_vec.push(
            Row::new(curr_row)
                .height(1)
                .style(Style::new().bg(match i % 2 {
                    0 => tailwind::SLATE.c900,
                    _ => tailwind::SLATE.c950,
                })),
        );
    }
    app_state.display_track_list.1 = track_list_vec;
    app_state.display_track_list.2 = t_id_vec;
}

fn ui(app_state: &mut AppState, frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    // top and bottom panels
    frame.render_widget(
        Block::new().borders(Borders::TOP).title(format!(
            " {} - v{} ",
            std::env!("CARGO_PKG_NAME"),
            std::env!("CARGO_PKG_VERSION"),
        )),
        main_layout[0],
    );

    frame.render_widget(
        Block::new().borders(Borders::TOP).title("keybinds go here"),
        main_layout[2],
    );

    // left sidebar
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(20), Constraint::Percentage(80)],
    )
    .split(main_layout[1]);

    let filter_block = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ],
    )
    .split(inner_layout[0]);

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

    frame.render_stateful_widget(
        list,
        filter_block[0],
        &mut app_state.filter_filter_options.0,
    );

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

    frame.render_stateful_widget(
        filter_list,
        filter_block[1],
        &mut app_state.filter_options.0,
    );

    frame.render_widget(Block::new().borders(Borders::ALL), filter_block[2]);

    // main tracklist
    let widths = [
        Constraint::Percentage(40),
        Constraint::Percentage(2),
        Constraint::Percentage(25),
        Constraint::Percentage(2),
        Constraint::Percentage(17),
        Constraint::Percentage(2),
        Constraint::Percentage(7),
        Constraint::Percentage(5),
    ];

    let mut curr_track_list_name = match app_state
        .filter_options
        .1
        .get(app_state.filter_options.0.selected().unwrap())
    {
        Some(x) => x.clone(),
        None => "".to_string(),
    };

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
                }))
        )
        .header(
            Row::new(vec![
                "Title", "", "Artist", "", "Album", "", "Playlist", "Duration",
            ])
            .style(
                Style::new()
                    .fg(tailwind::SLATE.c200)
                    .bg(tailwind::BLUE.c900)
                    .add_modifier(Modifier::BOLD),
            )
            .height(1)
        )
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().bg(tailwind::BLUE.c400).fg(Color::Black));

    frame.render_stateful_widget(
        track_table,
        inner_layout[1],
        &mut app_state.display_track_list.0,
    );
}
