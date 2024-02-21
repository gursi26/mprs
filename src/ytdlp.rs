use std::process::{Command, Stdio};
use std::path::PathBuf;

static N_OUTPUT_ITEMS: i8 = 5;

pub fn search_ytdlp(
    query: &String,
    n_results: i8,
) -> (Vec<String>, Vec<(String, String, String, String)>) {
    let output = Command::new("yt-dlp")
        .arg(query)
        .arg("--get-title")
        .arg("--default-search")
        .arg(format!("ytsearch{}", n_results))
        .arg("--get-duration")
        .arg("--print")
        .arg("uploader")
        .arg("--print")
        .arg("upload_date")
        .arg("--get-id")
        .output()
        .expect("Search failed. Try again lol.");

    let string_output = String::from_utf8_lossy(&output.stdout);
    let parts = string_output.split("\n").collect::<Vec<&str>>();

    let mut id_vec: Vec<String> = Vec::new();
    let mut search_results: Vec<(String, String, String, String)> = Vec::new();

    for i in 0..n_results {
        let artist = String::from(parts[(N_OUTPUT_ITEMS * i) as usize]);
        let upload_date = String::from(parts[(N_OUTPUT_ITEMS * i + 1) as usize]);
        let name = String::from(parts[(N_OUTPUT_ITEMS * i + 2) as usize]);
        let id = String::from(parts[(N_OUTPUT_ITEMS * i + 3) as usize]);
        let duration = String::from(parts[(N_OUTPUT_ITEMS * i + 4) as usize]);

        let formatted_upload_date = format!(
            "{}-{}-{}",
            &upload_date[..4],
            &upload_date[4..6],
            &upload_date[6..]
        );

        id_vec.push(id);
        search_results.push((name, artist, duration, formatted_upload_date));
    }

    (id_vec, search_results)
}

pub fn ytdlp_download(
    video_id: &String,
    audio_format: &String,
    dest: &PathBuf
) -> bool {
    println!("Downloading...");
    let output = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg(audio_format)
        .arg("-P")
        .arg(format!("/{}", dest.to_str().unwrap()))
        .arg("--")
        .arg(video_id)
        .output();

    if let Ok(success) = output {
        // println!("{}", String::from_utf8_lossy(&success.stdout));
        return true;
    }
    false
}
