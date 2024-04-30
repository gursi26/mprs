use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::remove_file;
use crate::utils::{get_cache_file_path, get_metadata, get_music_dir, get_newtracks_dir};
use std::{collections::HashMap, fs::{read_dir, File, OpenOptions}, io::{Write, Read}, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: u32,
    pub name: String,
    pub artists: Option<Vec<String>>,
    pub album: Option<String>,
    pub playlist: String,
    pub duration: u32,
}

impl TrackInfo {
    pub fn get_file_name(&self) -> String {
        format!("{} - {}.mp3", self.name, self.id)
    }

    pub fn get_file_path(&self) -> PathBuf {
        let mut p = get_music_dir();
        p.push(self.get_file_name());
        p
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackDB {
    pub track_filter_cache: HashMap<String, HashMap<String, Vec<u32>>>,
    pub trackmap: HashMap<u32, TrackInfo>,
    pub max_id: u32,
}

impl TrackDB {
    pub fn init() -> Self {
        let mut tdb = TrackDB::new();
        let cache_path = get_cache_file_path();
        if cache_path.exists() {
            tdb.load_from_file();
        }
        tdb
    }

    pub fn new() -> Self {
        let mut m = HashMap::new();

        let mut hm: HashMap<String, Vec<u32>> = HashMap::new();
        hm.insert("Liked".to_string(), Vec::new());
        m.insert("Playlists".to_string(), hm);

        let mut hm: HashMap<String, Vec<u32>> = HashMap::new();
        hm.insert("None".to_string(), Vec::new());
        m.insert("Albums".to_string(), hm);

        let mut hm: HashMap<String, Vec<u32>> = HashMap::new();
        hm.insert("None".to_string(), Vec::new());
        m.insert("Artists".to_string(), hm);

        TrackDB {
            track_filter_cache: m,
            trackmap: HashMap::new(),
            max_id: 0,
        }
    }

    pub fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(get_cache_file_path())
            .unwrap();

        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        file.write_all(&encoded).unwrap();
    }

    pub fn load_from_file(&mut self) {
        let mut file = File::open(get_cache_file_path()).unwrap();
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).unwrap();

        let decoded: Self = bincode::deserialize(&encoded).unwrap();
        *self = decoded;
    }

    // Adds all tracks in the newtracks directory to DB
    pub fn add_all_tracks(&mut self, playlist: Option<String>) {
        let newtracks_path = get_newtracks_dir();
        let newtracks = read_dir(&newtracks_path).unwrap();
        for t in newtracks {
            let tp = t.unwrap().path();
            if tp.file_name().unwrap().to_str().unwrap().starts_with(".") {
                continue;
            }
            self.add_track_helper(&tp, playlist.clone());
        }
        self.save_to_file();
    }

    fn add_track_helper(&mut self, track_path: &PathBuf, playlist: Option<String>) {
        let new_track_id = self.max_id + 1;
        self.max_id = new_track_id;

        let mut name: String;
        let mut artists: Option<Vec<String>>;
        let mut album: Option<String>;
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
            playlist: playlist.clone().unwrap_or("Liked".to_string()),
            artists: artists.clone(),
            album: album.clone(),
        };

        let save_file_name = t_info.get_file_name();
        debug!("Adding track : {:?}", t_info);
        self.trackmap.insert(new_track_id, t_info);

        // update playlist_map
        let playlist_map = self.track_filter_cache.get_mut("Playlists").unwrap();
        match playlist {
            Some(p) => {
                if playlist_map.contains_key(&p) {
                    playlist_map.get_mut(&p).unwrap().push(new_track_id);
                } else {
                    playlist_map.insert(p, vec![new_track_id]);
                }
            }
            None => {
                let v = playlist_map.get_mut("Liked").unwrap();
                v.push(new_track_id);
            }
        }

        // update artist_map
        let artist_map = self.track_filter_cache.get_mut("Artists").unwrap();
        match artists {
            Some(ar) => {
                for a in ar.iter() {
                    if artist_map.contains_key(a) {
                        artist_map.get_mut(a).unwrap().push(new_track_id);
                    } else {
                        artist_map.insert(a.clone(), vec![new_track_id]);
                    }
                }
            },
            None => {
                let v = artist_map.get_mut("None").unwrap();
                v.push(new_track_id);
            }
        }

        // update album map
        let album_map = self.track_filter_cache.get_mut("Albums").unwrap();
        match album {
            Some(a) => {
                if album_map.contains_key(&a) {
                    album_map.get_mut(&a).unwrap().push(new_track_id);
                } else {
                    album_map.insert(a, vec![new_track_id]);
                }
            },
            None => {
                let v = album_map.get_mut("None").unwrap();
                v.push(new_track_id);
            }
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

        debug!("Updated filter cache!")

    }

    pub fn remove_track(&mut self, track_id: u32) {
        let t_info = self.trackmap.remove(&track_id).unwrap();
        let t_id = t_info.id;

        let playlist_map = self.track_filter_cache.get_mut("Playlists").unwrap();
        let p = playlist_map.get_mut(&t_info.playlist).unwrap();
        p.retain(|&x| x != t_id);

        let album_map = self.track_filter_cache.get_mut("Albums").unwrap();
        let p = album_map.get_mut(&t_info.album.clone().unwrap_or("None".to_string())).unwrap();
        p.retain(|&x| x != t_id);

        let artist_map = self.track_filter_cache.get_mut("Artists").unwrap();
        match t_info.artists.clone() {
            Some(ar) => {
                for a in ar.iter() {
                    let p = artist_map.get_mut(a).unwrap();
                    p.retain(|&x| x != t_id);
                }
            },
            None => {
                let p = artist_map.get_mut(&"None".to_string()).unwrap();
                p.retain(|&x| x != t_id);
            }
        }

        remove_file(t_info.get_file_path()).unwrap();
        self.save_to_file();
    }
}
