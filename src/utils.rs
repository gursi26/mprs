use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    process::{exit, Command, Stdio},
    str::FromStr,
};

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
