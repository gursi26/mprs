use crate::utils::{get_metadata, get_newtracks_dir};
use std::{collections::HashMap, fs::read_dir, path::PathBuf};

#[derive(Debug)]
pub struct TrackInfo {
    pub id: u32,
    pub name: String,
    pub artists: Vec<String>,
    pub album: String,
    pub playlist: String,
    pub duration: u32,
}

impl TrackInfo {
    pub fn get_save_file_name(&self) -> String {
        format!("{} - {}.mp3", self.name, self.id)
    }
}

#[derive(Debug)]
pub struct TrackDB {
    pub trackmap: HashMap<String, HashMap<String, Vec<u32>>>,
    pub tracklist: Vec<TrackInfo>,
    pub trackid_to_idx: HashMap<u32, usize>,
    pub max_id: u32,
}

impl TrackDB {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("Playlists".to_string(), HashMap::new());
        m.insert("Albums".to_string(), HashMap::new());
        m.insert("Artists".to_string(), HashMap::new());
        TrackDB {
            trackmap: m,
            tracklist: Vec::new(),
            trackid_to_idx: HashMap::new(),
            max_id: 0,
        }
    }

    // Adds all tracks in the newtracks directory to DB
    pub fn add_track(&mut self, playlist: String) {
        let newtracks_path = get_newtracks_dir();
        let newtracks = read_dir(&newtracks_path).unwrap();
        for t in newtracks {
            let tp = t.unwrap().path();
            if tp.file_name().unwrap().to_str().unwrap().starts_with(".") {
                continue;
            }
            self.add_track_helper(&tp, playlist.clone());
        }
    }

    fn add_track_helper(&mut self, track_path: &PathBuf, playlist: String) {
        let new_track_id = self.max_id + 1;
        self.max_id = new_track_id;

        let mut name: String;
        let mut artists: Vec<String>;
        let mut album: String;
        let mut duration: u32;
        if let Some((n, ar, al, d)) = get_metadata(track_path) {
            name = n;
            artists = ar;
            album = al;
            duration = d;
        } else {
            panic!("Could not read metadata");
        }

        let t_info = TrackInfo {
            id: new_track_id,
            name,
            duration,
            playlist: playlist.clone(),
            artists: artists.clone(),
            album: album.clone(),
        };

        let save_file_name = t_info.get_save_file_name();
        self.tracklist.push(t_info);
        self.trackid_to_idx
            .insert(new_track_id, self.tracklist.len() - 1);

        // update playlist_map
        let playlist_map = self.trackmap.get_mut("Playlists").unwrap();
        if playlist_map.contains_key(&playlist) {
            playlist_map.get_mut(&playlist).unwrap().push(new_track_id);
        } else {
            playlist_map.insert(playlist, vec![new_track_id]);
        }

        // update artist_map
        let artist_map = self.trackmap.get_mut("Artists").unwrap();
        for a in artists.iter() {
            if artist_map.contains_key(a) {
                artist_map.get_mut(a).unwrap().push(new_track_id);
            } else {
                artist_map.insert(a.clone(), vec![new_track_id]);
            }
        }

        // update album map
        let album_map = self.trackmap.get_mut("Albums").unwrap();
        if album_map.contains_key(&album) {
            album_map.get_mut(&album).unwrap().push(new_track_id);
        } else {
            album_map.insert(album, vec![new_track_id]);
        }

        // move file from newtracks dir
        let mut save_path = track_path
            .clone()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        save_path.push(save_file_name);
        std::fs::rename(track_path, save_path).unwrap();
    }
}
