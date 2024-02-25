use crate::args::*;
use crate::config::*;
use crate::ytdlp::*;
use std::fs::rename;
use std::io::{stdout, Write};
use crate::utils::{set_artist, print_table};

static N_RESULTS_PER_THREAD: i16 = 3;

pub async fn mprs_add(args: &AddArgs, config: &UserConfig) {
    println!(
        "Query: \"{}\"\nPlaylist: {}",
        args.query_term, args.playlist
    );

    let (id_vec, results_vec) = search_ytdlp(&args.query_term, N_RESULTS_PER_THREAD).await;
    print_table(&results_vec);

    print!("Select song by number : ");
    let _ = stdout().flush();

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    let id_idx: i32 = input_string.trim().parse().unwrap();

    let mut save_path = config.base_dir.clone();
    save_path.push(&args.playlist);
    let video_id = &id_vec[(id_idx - 1) as usize];

    if ytdlp_download(
        video_id,
        &config.audio_format,
        &save_path,
    ) {
        println!("Successfully downloaded to {:?}", save_path);
    }

    // FIX: Deal with '/' in song name file path bug
    let mut song_name = results_vec[id_idx as usize][0].clone();
    for i in 1..song_name.len() {
        if song_name.as_bytes()[i] == '/' as u8 {
            song_name.insert(i - 1, '\\');
        }
    }
    println!("{}", song_name);
    save_path.push(format!("mprs-audio.{}", config.audio_format));

    let channel_name = &results_vec[id_idx as usize][1];
    set_artist(&save_path, &channel_name);

    let old_save_path = save_path.clone();
    save_path.pop();
    save_path.push(format!("{}.{}", song_name, config.audio_format));

    rename(old_save_path, save_path).unwrap();
}
