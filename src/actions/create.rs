use crate::args::*;
use crate::config::*;
use std::fs::create_dir;

pub fn mprs_create(args: &CreateArgs, config: &UserConfig) {
    let mut playlist_dir = config.base_dir.clone();
    playlist_dir.push(&args.playlist_name);
    let _ = create_dir(&playlist_dir);
    println!("Created playlist at {:?}", playlist_dir);
}
