use std::{fs::{create_dir_all, File}, io::Write, path::PathBuf, str::FromStr};

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
