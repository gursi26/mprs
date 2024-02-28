use std::process::Stdio;
use std::path::PathBuf;
use std::error::Error;
// use tokio::process::Command;
use tokio::try_join;
use std::process::Command;
use std::thread;

static N_OUTPUT_ITEMS: i16 = 5;

pub fn search_ytdlp(
    query: &String,
    n_results: i16,
) -> (Vec<String>, Vec<Vec<String>>) {
    let mut parts = threaded_search(query, n_results as i32).unwrap();
    parts.retain(|x| *x != String::from(""));

    let mut id_vec: Vec<String> = Vec::new();
    let mut search_results: Vec<Vec<String>> = Vec::new();

    search_results.push(vec![
        String::from("Name"),
        String::from("Artist"),
        String::from("Duration"),
        String::from("Upload Date"),
    ]);

    for i in 0..(parts.len() as i16 / N_OUTPUT_ITEMS) {
        let artist = parts[(N_OUTPUT_ITEMS * i) as usize].clone();
        let upload_date = parts[(N_OUTPUT_ITEMS * i + 1) as usize].clone();
        let name = parts[(N_OUTPUT_ITEMS * i + 2) as usize].clone();
        let id = parts[(N_OUTPUT_ITEMS * i + 3) as usize].clone();
        let duration = parts[(N_OUTPUT_ITEMS * i + 4) as usize].clone();


        let formatted_upload_date = format!(
            "{}-{}-{}",
            &upload_date[..4],
            &upload_date[4..6],
            &upload_date[6..]
        );

        if !id_vec.contains(&id) {
            id_vec.push(id);
            search_results.push(vec![name, artist, duration, formatted_upload_date]);
        }
    }

    (id_vec, search_results)
}

fn threaded_search(query_term: &String, n_results: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let query_term_full = [
        " audio",
        " official",
        " official audio",
        " official music video",
        " music video",
        " lyrics",
        " lyric video",
        " full song",
        " full audio",
        " full lyric video",
    ]
    .map(|x| format!("{}{}", query_term, x));

    let mut handles = Vec::new();
    for query_term in query_term_full {
        let handle = thread::spawn(move || {
            let output = Command::new("yt-dlp")
                .arg(query_term)
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
                .unwrap();

            // Convert the output to a String
            let string_output = String::from_utf8_lossy(&output.stdout);
            let parts = string_output
                .split("\n")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            parts
        });
        handles.push(handle);
    }

    let mut outputs = Vec::new();
    for handle in handles {
        let mut output = handle.join().unwrap();
        outputs.append(&mut output);
    }
    Ok(outputs)
}

pub fn ytdlp_get_info_from_link(link: &String) -> Vec<(String, String, String)> {
    let output = std::process::Command::new("yt-dlp")
        .arg(link)
        .arg("--get-title")
        .arg("--default-search")
        .arg("ytsearch")
        .arg("--print")
        .arg("uploader")
        .arg("--get-id")
        .output()
        .unwrap();

    let string_output = String::from_utf8_lossy(&output.stdout);

    let mut parts = string_output
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    parts.retain(|x| x != "");
    let n_results = parts.len() / 3;
    let mut results: Vec<(String, String, String)> = Vec::new();

    for i in 0..n_results {
        let artist = parts[3 * i].clone();
        let track_name = parts[3 * i + 1].clone();
        let id = parts[3 * i + 2].clone();
        results.push((track_name, artist, id));
    }
    results
}


pub fn ytdlp_download(
    video_id: &String,
    audio_format: &String,
    dest: &PathBuf
) -> bool {
    println!("Downloading...");
    let output = std::process::Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg(audio_format)
        .arg("--output")
        .arg("mprs-audio.%(ext)s")
        .arg("-P")
        .arg(format!("/{}", dest.to_str().unwrap()))
        .arg("--")
        .arg(video_id)
        .output();

    if let Ok(success) = output {
        return true;
    }
    false
}
