use crate::{args::PlayArgs, utils::get_artist};
use crate::config::UserConfig;
use mprs::utils::{base_dir, list_dir};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use stopwatch::Stopwatch;

use crate::utils::{get_duration, get_input_key, get_instruction_string, UserInput};

use anyhow::Result;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::{CrosstermBackend, Frame, Terminal};
use ratatui::{prelude::*, widgets::*};

static TIME_UNTIL_BACK_SELF: u64 = 3;

struct App {
    songs: Vec<(String, String, String, String)>,
    curr_song: String,
    curr_artist: String,
    curr_playlist: String,
    start_time: Stopwatch,
    end_of_song: bool,
    percent_of_song_complete: f64,
    curr_song_base_duration: u64,
    curr_song_adjusted_duration: u64,
    curr_volume: f32,
    curr_speed: f32,
    instruction_string: String,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, frame: &mut Frame) {
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
        Block::new().borders(Borders::TOP).title(" mprs "),
        main_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(app.instruction_string.clone()),
        main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);

    let visualizer_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .split(inner_layout[1]);

    let mut rows: Vec<Row> = Vec::new();
    rows.push(Row::new(vec![
            String::from("        \u{25b6}"),
            app.songs[0].0.clone(),
            app.songs[0].1.clone(),
            app.songs[0].2.clone(),
            app.songs[0].3.clone(),
        ]).style(Style::new().blue().bold()));

    for (i, x) in app.songs.iter().skip(1).enumerate() {
        rows.push(Row::new(vec![
            format!("{}", i + 1),
            x.0.clone(),
            x.1.clone(),
            x.2.clone(),
            x.3.clone(),
        ]))
    }

    let widths = [
        Constraint::Length(10),
        Constraint::Length(170),
        Constraint::Length(40),
        Constraint::Length(15),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["Position", "Name", "Artist", "Playlist", "Duration"]).style(Style::new().bold()),
        )
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

    frame.render_widget(
        table.block(Block::default().borders(Borders::ALL).title("Queue")),
        inner_layout[0],
    );

    let now_playing_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(60), Constraint::Percentage(40)],
    )
    .split(visualizer_layout[0]);

    let seconds = app.curr_song_adjusted_duration;
    let s1 = format!("{}:{:0>2}", seconds / 60, seconds - ((seconds / 60) * 60));
    let elapsed = app.start_time.elapsed().as_secs();
    let s2 = format!("{}:{:0>2}", elapsed / 60, elapsed - ((elapsed / 60) * 60));
    frame.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL))
            .ratio(app.percent_of_song_complete.clamp(0.0, 1.0))
            .label(format!("{} / {}", s2, s1))
            .use_unicode(true),
        now_playing_layout[1],
    );

    let text = vec![
        Line::from(format!("{}", app.curr_song.clone()))
            .style(Style::new().yellow().bold())
            .centered(),
        Line::from(" "),
        Line::from(format!("Artist : {}", app.curr_artist.clone()))
            .style(Style::new())
            .centered(),
        Line::from(format!("Playlist : {}", app.curr_playlist.clone()))
            .style(Style::new())
            .centered(),
    ];

    frame.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: true }).block(
            Block::new()
                .title("Now Playing")
                .borders(Borders::ALL)
                .padding(Padding {
                    left: 2,
                    right: 2,
                    top: 1,
                    bottom: 1,
                }),
        ),
        now_playing_layout[0],
    );

    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Visualizer"),
        visualizer_layout[1],
    );
}

fn table_from_song_queue(song_queue: &[PathBuf]) -> Vec<(String, String, String, String)> {
    let mut output_vec = Vec::new();
    for p in song_queue.iter() {
        let song_path = p.clone();
        let song_name = song_path.as_path().file_name().unwrap().to_str().unwrap();
        let artist_name = get_artist(&song_path);
        let playlist_name = song_path
            .as_path()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let seconds = get_duration(&song_path);
        let duration_str = format!("{}:{:0>2}", seconds / 60, seconds - ((seconds / 60) * 60));
        output_vec.push((
            String::from(song_name),
            artist_name,
            String::from(playlist_name),
            duration_str,
        ));
    }
    output_vec
}

fn update(app: &mut App, sink: &Sink, curr_idx: &mut i32, song_queue: &Vec<PathBuf>) -> Result<()> {
    app.curr_song_adjusted_duration = (app.curr_song_base_duration as f32 / app.curr_speed) as u64;
    app.songs = table_from_song_queue(&song_queue[((*curr_idx - 1).clamp(0, i32::MAX) as usize)..]);
    app.percent_of_song_complete =
        app.start_time.elapsed().as_secs() as f64 / app.curr_song_adjusted_duration as f64;
    sink.set_speed(app.curr_speed);
    sink.set_volume(app.curr_volume);
    match get_input_key() {
        UserInput::Quit => app.should_quit = true,
        UserInput::Next => {
            sink.stop();
            app.end_of_song = true;
        }
        UserInput::Previous => {
            *curr_idx -= 1;
            if app.start_time.elapsed().as_secs() < TIME_UNTIL_BACK_SELF {
                *curr_idx -= 1;
            }
            sink.stop();
            app.end_of_song = true;
        }
        UserInput::Pause => {
            if sink.is_paused() {
                sink.play();
                app.start_time.start();
            } else {
                sink.pause();
                app.start_time.stop();
            }
        }
        UserInput::VolumeUp => app.curr_volume += 0.1,
        UserInput::VolumeDown => app.curr_volume -= 0.1,
        UserInput::SpeedUp => app.curr_speed += 0.1,
        UserInput::SpeedDown => app.curr_speed -= 0.1,
        UserInput::ResetSpeed => app.curr_speed = 1.0,
        UserInput::DoNothing => {}
    };
    Ok(())
}

fn run(sink: &Sink, song_queue: Vec<PathBuf>) -> Result<()> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // application state
    let mut app = App {
        songs: Vec::new(),
        curr_song: String::new(),
        curr_artist: String::new(),
        curr_playlist: String::new(),
        start_time: Stopwatch::start_new(),
        curr_speed: 1.0,
        curr_volume: 1.0,
        end_of_song: false,
        instruction_string: get_instruction_string(),
        should_quit: false,
        curr_song_base_duration: 0,
        curr_song_adjusted_duration: 0,
        percent_of_song_complete: 0.0,
    };

    let mut i: i32 = 0;
    while i >= 0 && i <= (song_queue.len() as i32) {
        if sink.empty() {
            if i >= (song_queue.len() as i32) {
                break;
            }
            let curr_song = &song_queue[i as usize];
            sink.append(Decoder::new(BufReader::new(File::open(curr_song).unwrap())).unwrap());
            app.start_time = Stopwatch::start_new();
            app.curr_song_base_duration = get_duration(curr_song);
            app.curr_song_adjusted_duration = app.curr_song_base_duration;
            app.curr_song = curr_song
                .as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            app.curr_playlist = curr_song
                .as_path()
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            app.curr_artist = get_artist(&curr_song);
            i += 1;
        }

        update(&mut app, sink, &mut i, &song_queue).unwrap();

        t.draw(|f| {
            ui(&app, f);
        })?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

pub fn mprs_play(args: &PlayArgs, config: &UserConfig) {
    let playlists = list_dir(&config.base_dir);
    let current_song: PathBuf;
    let mut song_queue: Vec<PathBuf>;

    match (&args.query_term, &args.playlist) {
        // Case where playlist is specified
        (q, Some(p)) => {
            let mut selected_playlist = base_dir();
            selected_playlist.push(p.clone());

            if !playlists.contains(&selected_playlist) {
                println!("Playlist {:?} not found", selected_playlist);
                return;
            }
            song_queue = list_dir(&selected_playlist);

            match q {
                // case where song is specified
                Some(query) => {
                    let mut start_index = -1;
                    for (i, x) in song_queue.iter().enumerate() {
                        if x.as_path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .contains(&query.to_lowercase()[..])
                        {
                            start_index = i as i32;
                        }
                    }

                    if start_index == -1 {
                        println!("\"{}\" not found in playlist \"{}\"", query, p);
                        return;
                    }

                    current_song = song_queue.remove(start_index as usize);
                    if args.shuffle {
                        song_queue.shuffle(&mut thread_rng());
                    }
                    song_queue.insert(0, current_song);
                }
                _ => {
                    if args.shuffle {
                        song_queue.shuffle(&mut thread_rng());
                    }
                }
            }
        }
        // Case where no playlist is specified
        (q, None) => {
            song_queue = Vec::new();
            for p in list_dir(&config.base_dir) {
                song_queue.extend(list_dir(&p));
            }

            match q {
                Some(query) => {
                    let mut start_index = -1;
                    for (i, x) in song_queue.iter().enumerate() {
                        if x.as_path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .contains(&query.to_lowercase()[..])
                        {
                            start_index = i as i32;
                        }
                    }

                    if start_index == -1 {
                        println!("\"{}\" not found", query);
                        return;
                    }
                    current_song = song_queue.remove(start_index as usize);

                    if args.shuffle {
                        song_queue.shuffle(&mut thread_rng());
                    }
                    song_queue.insert(0, current_song);
                }
                _ => {
                    if args.shuffle {
                        song_queue.shuffle(&mut thread_rng());
                    }
                }
            }
        }
    };

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    startup().unwrap();
    let result = run(&sink, song_queue);
    shutdown().unwrap();
    result.unwrap();
}
