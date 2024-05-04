use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    process::{exit, Command, Stdio},
    str::FromStr,
};

use crossterm::event::{KeyCode, KeyModifiers};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;

use dirs::home_dir;

use crate::{MPV_LUASCRIPT_FILENAME, MPV_STATUS_IPC_FILENAME, MUSIC_DIR};

pub fn get_music_dir() -> PathBuf {
    let mut d = home_dir().unwrap();
    d.push(MUSIC_DIR);
    d
}

pub fn get_newtracks_dir() -> PathBuf {
    let mut mdir = get_music_dir();
    mdir.push("newtracks");
    mdir
}

pub fn get_cache_file_path() -> PathBuf {
    let mut mdir = get_music_dir();
    mdir.push(".trackdb");
    mdir
}

pub fn get_ipc_path() -> PathBuf {
    let mut p = get_music_dir();
    p.push(MPV_STATUS_IPC_FILENAME);
    p
}

pub fn get_luascript_path() -> PathBuf {
    let mut p = get_music_dir();
    p.push(MPV_LUASCRIPT_FILENAME);
    p
}

pub fn init_functions() {
    init_files();
    check_spotdl_installed();
    // setup_logger().unwrap();
}

pub fn init_files() {
    // creates directory for music
    let mut music_dir_path = get_music_dir();
    create_dir_all(&music_dir_path).unwrap();

    // creates ipc file for communication with mpv
    let mut ipc_path = get_ipc_path();
    File::create(&ipc_path).unwrap();

    // creates lua script for mpv
    let mut luascript_path = get_luascript_path();
    init_lua_file(&luascript_path, ipc_path.to_str().unwrap());
}

fn init_lua_file(fp: &PathBuf, ipc_path: &str) {
    let mut file = File::create(fp).unwrap();
    let mut file_contents = include_str!("status_update.lua").to_string();
    file_contents = file_contents.replace("<script_path>", ipc_path);
    file.write_all(file_contents.as_bytes()).unwrap();
}

pub fn parse_bool(s: &str) -> bool {
    if s == "true" {
        true
    } else {
        false
    }
}

pub fn check_spotdl_installed() {
    let out = String::from_utf8(Command::new("pip").arg("list").output().unwrap().stdout).unwrap();
    for l in out.lines() {
        if l.contains("spotdl") {
            return;
        }
    }
    eprintln!("spotdl installation not found! If you are using conda/venv, ensure that you are in the correct environment!");
    exit(1);
}

pub fn get_progress_display_str(secs_played: f64, total_secs: f64) -> String {
    let (s1, s2) = (secs_played as u32, total_secs as u32);
    let (m1, m2) = (s1 / 60, s2 / 60);
    let (ss1, ss2) = (s1 - m1 * 60, s2 - m2 * 60);
    format!("{}:{:0>2}/{}:{:0>2}", m1, ss1, m2, ss2)
}

pub fn get_metadata(p: &PathBuf) -> Option<(String, Option<Vec<String>>, Option<String>, u32)> {
    let tagged_file = Probe::open(p).unwrap().read().unwrap();
    let duration = tagged_file.properties().duration().as_secs() as u32;

    if let Some(tag) = tagged_file.primary_tag() {
        let title = tag.title().unwrap().as_ref().to_string();
        let album = match tag.album() {
            Some(x) => Some(x.as_ref().to_string()),
            None => None,
        };
        let artist = match tag.artist() {
            Some(x) => Some(
                x.as_ref()
                    .to_string()
                    .split("/")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
            None => None,
        };
        Some((title, artist, album, duration))
    } else {
        None
    }
}

pub fn wrap_string(s: &mut String, n: u32) {
    let c = s.len() as u32 / n;
    for i in 0..c {
        s.insert((c * n) as usize, '\n');
    }
}

// TODO: Fix this function
pub fn set_metadata(
    p: &PathBuf,
    title: String,
    artists: Vec<String>,
    album: String,
) -> Option<(String, Option<Vec<String>>, Option<String>, u32)> {
    let tagged_file = Probe::open(p).unwrap().read().unwrap();
    let duration = tagged_file.properties().duration().as_secs() as u32;

    if let Some(tag) = tagged_file.primary_tag() {
        let title = tag.title().unwrap().as_ref().to_string();
        let album = match tag.album() {
            Some(x) => Some(x.as_ref().to_string()),
            None => None,
        };
        let artist = match tag.artist() {
            Some(x) => Some(vec![x.as_ref().to_string()]),
            None => None,
        };
        Some((title, artist, album, duration))
    } else {
        None
    }
}

pub fn get_album_cover(p: &PathBuf) -> Vec<u8> {
    let tagged_file = Probe::open(p).unwrap().read().unwrap();

    if let Some(tag) = tagged_file.primary_tag() {
        tag.pictures()[0].clone().into_data()
    } else {
        Vec::new()
    }
}


pub enum UserInput {
    Quit,
    DoNothing,
    FocusLower,
    FocusUpper,
    FocusLeft,
    FocusRight,
    SelectLower,
    SelectUpper,
    Select,
    JumpToBottom,
    JumpToTop,
    JumpMultipleDown,
    JumpMultipleUp
}

pub fn get_input_key() -> UserInput {
    if crossterm::event::poll(std::time::Duration::from_millis(250)).unwrap() {
        // If a key event occurs, handle it
        if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => UserInput::FocusLeft,
                    (KeyCode::Char('j'), KeyModifiers::CONTROL) => UserInput::FocusLower,
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => UserInput::FocusUpper,
                    (KeyCode::Char('l'), KeyModifiers::CONTROL) => UserInput::FocusRight,
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => UserInput::JumpMultipleDown,
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => UserInput::JumpMultipleUp,
                    (KeyCode::Char('G'), KeyModifiers::SHIFT) => UserInput::JumpToBottom,
                    (KeyCode::Char('q'), _) => UserInput::Quit,
                    (KeyCode::Char('g'), _) => UserInput::JumpToTop,
                    (KeyCode::Char('j'), _) => UserInput::SelectLower,
                    (KeyCode::Char('k'), _) => UserInput::SelectUpper,
                    (KeyCode::Enter, _) => UserInput::Select,
                    _ => UserInput::DoNothing,
                }
            } else {
                UserInput::DoNothing
            }
        } else {
            UserInput::DoNothing
        }
    } else {
        UserInput::DoNothing
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level_for("mprs", log::LevelFilter::Debug)
        .level(log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
