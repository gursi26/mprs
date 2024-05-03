use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Frame, Terminal};
use ratatui::{prelude::*, widgets::*};
use std::sync::{Arc, Mutex};

use crate::{
    state::{AppState, FocusedWindow},
    utils::{get_input_key, UserInput},
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
            },
            FocusedWindow::FilterOptions => {
                let s_idx = app_state.filter_options.0.selected_mut();
                *s_idx =
                    Some((s_idx.unwrap() + 1).min(app_state.filter_options.1.len() - 1));
            }
            _ => {}
        },
        UserInput::SelectUpper => match app_state.focused_window {
            FocusedWindow::FilterFilterOptions => {
                let s_idx = app_state.filter_filter_options.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i8 - 1).max(0) as usize);

                let s2_idx = app_state.filter_options.0.selected_mut();
                *s2_idx = Some(0);
            },
            FocusedWindow::FilterOptions => {
                let s_idx = app_state.filter_options.0.selected_mut();
                *s_idx = Some((s_idx.unwrap() as i8 - 1).max(0) as usize);
            }
            _ => {}
        },
        _ => {}
    }
}

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
            display_list.insert(0, "Liked".to_string());
        } else {
            display_list.retain(|x| x != "None");
            display_list.insert(0, "None".to_string());
        }

        app_state.filter_options.1 = display_list;
    } else {
        app_state.filter_options.1 = Vec::new();
    }
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

    let filter_list = List::new(app_state.filter_options.1.clone())
        .block(
            Block::default()
                .title(format!(
                    " {} ",
                    app_state.filter_filter_options.1
                        [app_state.filter_filter_options.0.selected().unwrap()]
                ))
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
}
