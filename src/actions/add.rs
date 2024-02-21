use crate::args::*;
use crate::config::*;
use crate::ytdlp::*;
use std::io::{stdout, Write};
use mprs::utils::print_table;

pub fn mprs_add(args: &AddArgs, config: &UserConfig) {
    // println!("{:?}", config);
    // println!("{:?}", args);
    println!(
        "Query: {}\nNumber of results: {}\nPlaylist: {}",
        args.query_term, args.count, args.playlist
    );

    let (id_vec, results_vec) = search_ytdlp(&args.query_term, args.count);
    print_table(&results_vec);

    print!("Select song by number : ");
    let _ = stdout().flush();

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    let id_idx: i32 = input_string.trim().parse().unwrap();

    let mut save_path = config.base_dir.clone();
    save_path.push(&args.playlist);

    if ytdlp_download(
        &id_vec[(id_idx - 1) as usize],
        &config.audio_format,
        &save_path,
    ) {
        println!("Successfully downloaded to {:?}", save_path);
    }
}
