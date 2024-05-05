use std::{path::PathBuf, process::{Command, Stdio}};
use log::debug;
use rspotify::{
    model::{AlbumId, SearchResult as rsptSearchResult, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};
use tokio::runtime::Runtime;

use std::fs::File;

use crate::utils::{get_music_dir, get_newtracks_dir};

#[derive(Debug)]
pub struct SearchResult {
    pub name: String,
    pub artists: Vec<String>,
    pub album: String,
    pub id: String,
    pub duration: u64,
}

impl SearchResult {
    pub fn get_url(&self) -> String {
        format!("http://open.spotify.com/track/{}", self.id)
    }
}

fn get_creds() -> (String, String) {
    let output = String::from_utf8(
        Command::new("pip")
            .arg("show")
            .arg("spotdl")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let mut location = PathBuf::new();
    for l in output.lines() {
        if l.contains("Location") {
            location.push(l.split(" ").collect::<Vec<&str>>()[1]);
        }
    }

    location.push("spotdl");
    location.push("utils");
    location.push("config.py");

    let mut client_id = String::new();
    let mut client_secret = String::new();

    let file_contents = std::fs::read_to_string(location).unwrap();
    for l in file_contents.lines() {
        if l.contains("client_id"){
            client_id.push_str(l.split("\"").collect::<Vec<&str>>()[3]);
        }
        if l.contains("client_secret") {
            client_secret.push_str(l.split("\"").collect::<Vec<&str>>()[3]);
        }
    }

    (client_id, client_secret)
}

pub fn init_spotify_client() -> ClientCredsSpotify {
    let (id, secret) = get_creds();
    let creds = Credentials {
        id,
        secret: Some(secret),
    };
    ClientCredsSpotify::new(creds)
}

pub fn search_tracks(search_string: String, n_results: u32, spotify: &mut ClientCredsSpotify) -> Vec<SearchResult> {
    debug!("Searching for query \'{}\'", &search_string);
    let rt = Runtime::new().unwrap();
    let results = rt.block_on(async {
        spotify.request_token().await.unwrap();
        spotify
            .search(
                &search_string[..],
                SearchType::Track,
                None,
                None,
                Some(n_results),
                None,
            )
            .await
            .unwrap()
    });

    let mut parsed_results = Vec::new();
    if let rsptSearchResult::Tracks(tracks) = results {
        for t in tracks.items.iter() {
            let mut artists = Vec::new();
            for v in &t.artists {
                artists.push(v.name.clone());
            }

            let id = t.id.clone()
                .unwrap()
                .to_string()
                .clone();

            let track_id = id.clone()
                .split(":")
                .collect::<Vec<&str>>()[2]
                .to_string();

            parsed_results.push(
                SearchResult {
                    name: t.name.clone(),
                    album: t.album.name.clone(),
                    artists,
                    id: track_id,
                    duration: t.duration.num_seconds() as u64
                }
            );
        }
    }
    parsed_results
}

pub fn download_track(url: &String) {
    debug!("Downloading track from url: {}", url);

    Command::new("spotdl")
        .arg(format!("{}", url))
        .arg("--format")
        .arg("mp3")
        .arg("--output")
        .arg(get_newtracks_dir().as_os_str().to_str().unwrap())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .unwrap();
}
