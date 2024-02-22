use crate::args::PlayArgs;
use crate::config::UserConfig;
use async_process::Command;
use mprs::utils::list_dir;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

pub fn mprs_play(args: &PlayArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);

    let playlists = list_dir(&config.base_dir);

    match (&args.query_term, &args.playlist) {
        (Some(q), Some(p)) => {}
        (Some(q), None) => {}
        (None, Some(p)) => {}
        (None, None) => {}
    };
}
