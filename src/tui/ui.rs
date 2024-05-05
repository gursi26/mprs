use crate::consts::*;
use ratatui::{prelude::*, widgets::*};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::state::app_state::*;
use style::palette::tailwind;
use crate::utils::*;
use stopwatch::Stopwatch;
use crate::tui::update::update;
use ratatui_image::StatefulImage;


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
    let list = List::new(app_state.filter_filter.options.clone())
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

    frame.render_stateful_widget(list, space, &mut app_state.filter_filter.state);
}

fn render_filter_options(frame: &mut Frame, app_state: &mut AppState, space: Rect) {
    let curr_selected_filter_filter = app_state.filter_filter.get_current_selection().unwrap();
    let filter_list = List::new(app_state.filter.options.clone())
        .block(
            Block::default()
                .title(match &curr_selected_filter_filter[..] {
                    "All" => "".to_string(),
                    _ => format!(
                        " {} ({}) ",
                        curr_selected_filter_filter,
                        app_state.filter.options.len()
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

    frame.render_stateful_widget(filter_list, space, &mut app_state.filter.state);
}

fn rows_from_vec(v: Vec<(u32, Vec<String>)>) -> Vec<Row<'static>> {
    v.into_par_iter()
        .map(|x| Row::new(x.1))
        .collect::<Vec<Row>>()
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

    let mut curr_track_list_name = app_state.filter.get_current_selection().unwrap_or("".to_string());
    let curr_selected_filter_filter = app_state.filter_filter.get_current_selection().unwrap();

    if "All" == &curr_selected_filter_filter[..] {
        curr_track_list_name = "All Tracks".to_string();
    }

    let rows = rows_from_vec(app_state.track_list.options.clone());
    let track_table = Table::new(rows, widths)
        .block(
            Block::default()
                .title(format!(
                    " {} ({}) ",
                    curr_track_list_name,
                    app_state.track_list.options.len()
                ))
                .borders(Borders::ALL)
                .border_style(Style::new().fg(match app_state.focused_window {
                    FocusedWindow::TrackList => SELECT_COLOR,
                    _ => UNSELECTED_COLOR,
                })),
        )
        .header(
            Row::new(vec![
                "ID", "Title", "", "Artist", "", "Album", "", "Playlist", "Duration",
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

    frame.render_stateful_widget(track_table, space, &mut app_state.track_list.state);
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

pub fn ui(app_state: &mut AppState, frame: &mut Frame) {
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
