use std::{fs::{create_dir_all, read_dir, File}, io::Write, path::{Path, PathBuf}, str::FromStr};
pub mod src::utils;

use dirs::home_dir;
extern crate chrono;
use chrono::{DateTime, Utc};
use comfy_table::*;
use lofty::{tag::Tag, prelude::{Accessor, AudioFile, TagExt, TaggedFileExt}, probe::Probe};
use std::collections::HashMap;
use db::TrackInfo;

use crate::{MUSIC_DIR, MPV_STATUS_IPC_FILENAME, MPV_LUASCRIPT_FILENAME};

pub fn get_music_dir() -> PathBuf {
    PathBuf::from_str(MUSIC_DIR).unwrap()
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

pub fn get_track_info(track_path: &PathBuf) -> TrackInfo {
    let info = get_track_information(track_path);
    TrackInfo {
        id: 0,
        name: info[0].clone(),
        artist: info[1].clone(),
        album: info[2].clone(),
        playlist: info[3].clone(),
        duration: info[4].parse().unwrap(),
    }
}

pub fn get_duration(path: &PathBuf) -> u64 {
    let duration = read_from_path(path.clone())
        .unwrap()
        .properties()
        .duration();
    duration.as_secs()
}

pub fn get_artist(path: &PathBuf) -> String {
    if !path.is_file() {
        panic!("ERROR: Path is not a file!");
    }

    let tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    tag.artist().as_deref().unwrap_or("Unknown").to_string()
}
