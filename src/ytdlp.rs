use std::process::Stdio;
use std::path::PathBuf;
use std::error::Error;
use tokio::process::Command;
use tokio::try_join;


static N_OUTPUT_ITEMS: i16 = 5;
static N_THREADS: i16 = 10;

pub async fn search_ytdlp(
    query: &String,
    n_results: i16,
) -> (Vec<String>, Vec<Vec<String>>) {
    let mut parts = threaded_search(query, n_results as i32).await.unwrap();
    parts.retain(|x| *x != String::from(""));

    let mut id_vec: Vec<String> = Vec::new();
    let mut search_results: Vec<Vec<String>> = Vec::new();

    search_results.push(vec![
        String::from("Name"),
        String::from("Artist"),
        String::from("Duration"),
        String::from("Upload Date"),
    ]);

    for i in 0..(n_results * N_THREADS) {
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

async fn threaded_search(query_term: &String, n_results: i32) -> Result<Vec<String>, Box<dyn Error>> {
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
    let mut results = try_join!(
        async { search_thread_spawn(&query_term_full[0], n_results).await },
        async { search_thread_spawn(&query_term_full[1], n_results).await },
        async { search_thread_spawn(&query_term_full[2], n_results).await },
        async { search_thread_spawn(&query_term_full[3], n_results).await },
        async { search_thread_spawn(&query_term_full[4], n_results).await },
        async { search_thread_spawn(&query_term_full[5], n_results).await },
        async { search_thread_spawn(&query_term_full[6], n_results).await },
        async { search_thread_spawn(&query_term_full[7], n_results).await },
        async { search_thread_spawn(&query_term_full[8], n_results).await },
        async { search_thread_spawn(&query_term_full[9], n_results).await },
    )?;

    let mut output = Vec::new();
    output.append(&mut results.0);
    output.append(&mut results.1);
    output.append(&mut results.2);
    output.append(&mut results.3);
    output.append(&mut results.4);
    output.append(&mut results.5);
    output.append(&mut results.6);
    output.append(&mut results.7);
    output.append(&mut results.8);
    output.append(&mut results.9);
    Ok(output)
}

async fn search_thread_spawn(query_term: &String, n_results: i32) -> Result<Vec<String>, Box<dyn Error>> {
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
        .await?;

    // Convert the output to a String
    let string_output = String::from_utf8_lossy(&output.stdout);
    let parts = string_output
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    Ok(parts)
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
