use crate::args::*;
use crate::config::*;
use crate::ytdlp::*;
use std::io::{stdout, Write};
use crate::utils::{set_artist, print_table};


pub async fn mprs_add(args: &AddArgs, config: &UserConfig) {
    println!(
        "Query: \"{}\"\nPlaylist: {}",
        args.query_term, args.playlist
    );

    let (id_vec, results_vec) = search_ytdlp(&args.query_term, 3).await;
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

    let song_name = &results_vec[id_idx as usize][0];
    let song_file_name = format!("{} [{}].{}", song_name, video_id, config.audio_format);
    save_path.push(song_file_name);

    let channel_name = &results_vec[id_idx as usize][1];
    set_artist(&save_path, &channel_name);
}
