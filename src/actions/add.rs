use mprs::utils::list_dir;

use crate::args::*;
use crate::config::*;
use crate::utils::base_dir;
use crate::utils::{print_table, set_artist};
use crate::ytdlp::*;
use std::fs::rename;
use std::io::{stdout, Write};

static N_RESULTS_PER_THREAD: i16 = 3;

pub fn mprs_add(args: &AddArgs, config: &UserConfig) {
    if !list_dir(&base_dir())
        .into_iter()
        .map(|x| {
            x.as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .contains(&args.playlist) {
        println!("Playlist {} not found.", &args.playlist);
        return;
    }

    println!(
        "Query: \"{}\"\nPlaylist: {}",
        args.query_term, args.playlist
    );

    let mut save_path = config.base_dir.clone();
    save_path.push(&args.playlist);

    if !args.query_term.starts_with("https") {
        let (id_vec, results_vec) = search_ytdlp(&args.query_term, N_RESULTS_PER_THREAD);
        print_table(&results_vec);

        print!("Select song by number : ");
        let _ = stdout().flush();

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        let id_idx: i32 = input_string.trim().parse().unwrap();

        let video_id = &id_vec[(id_idx - 1) as usize];
        let song_name = results_vec[id_idx as usize][0].clone().replace("/", "");

        if ytdlp_download(video_id, &config.audio_format, &save_path) {
            println!("Successfully downloaded \"{}\" to {:?}", &song_name, save_path);
        }

        save_path.push(format!("mprs-audio.{}", config.audio_format));

        let channel_name = &results_vec[id_idx as usize][1];
        set_artist(&save_path, &channel_name);

        let old_save_path = save_path.clone();
        save_path.pop();
        save_path.push(format!("{}.{}", song_name, config.audio_format));

        rename(old_save_path, save_path).unwrap();
    } else {
        println!("Searching...");
        let tracks_info = ytdlp_get_info_from_link(&args.query_term);

        for (track_name, artist, id) in tracks_info {
            if ytdlp_download(&id, &config.audio_format, &save_path) {
                println!("Successfully downloaded \"{}\" to {:?}", track_name, save_path);
            }

            save_path.push(format!("mprs-audio.{}", config.audio_format));
            set_artist(&save_path, &artist);

            let old_save_path = save_path.clone();
            save_path.pop();
            save_path.push(format!("{}.{}", track_name, config.audio_format));

            rename(old_save_path, &save_path).unwrap();
            save_path.pop();
        }
    }
}
