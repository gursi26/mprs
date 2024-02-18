use std::process::{Command, Stdio};

pub fn search_ytdlp(
    query: String,
    n_results: i8,
) -> (Vec<String>, Vec<(String, String, String)>) {
    println!("Query: {}\nNumber of results: {}", query, n_results);
    let output = Command::new("yt-dlp")
        .arg(query)
        .arg("--get-title")
        .arg("--default-search")
        .arg(format!("ytsearch{}", n_results))
        .arg("--get-duration")
        .arg("--print")
        .arg("uploader")
        .arg("--get-id")
        .output()
        .expect("Search failed. Try again lol.");

    let string_output = String::from_utf8_lossy(&output.stdout);
    let parts = string_output.split("\n").collect::<Vec<&str>>();

    let mut id_vec: Vec<String> = Vec::new();
    let mut search_results: Vec<(String, String, String)> = Vec::new();

    for i in 0..n_results {
        let artist = String::from(parts[(4 * i) as usize]);
        let name = String::from(parts[(4 * i + 1) as usize]);
        let id = String::from(parts[(4 * i + 2) as usize]);
        let duration = String::from(parts[(4 * i + 3) as usize]);

        id_vec.push(id);
        search_results.push((name, artist, duration));
    }

    (id_vec, search_results)
}
