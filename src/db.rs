use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::remove_file;
use crate::{state::filter_state::F1State, utils::{get_cache_file_path, get_metadata, get_music_dir, get_newtracks_dir}};
use std::{collections::BTreeMap, fs::{read_dir, File, OpenOptions}, io::{Write, Read}, path::PathBuf};

// change made here

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        let s = format!("{} - {}.mp3", self.name, self.id);
        s.replace("/", "")
    }

    pub fn get_file_path(&self) -> PathBuf {
        let mut p = get_music_dir();
        p.push(self.get_file_name());
        p
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackDB {
    pub track_filter_cache: BTreeMap<F1State, BTreeMap<String, Vec<u32>>>,
    pub trackmap: BTreeMap<u32, TrackInfo>,
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
        let mut m = BTreeMap::new();

        let mut hm: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        hm.insert("Liked".to_string(), Vec::new());
        m.insert(F1State::Playlists, hm);

        let mut hm: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        hm.insert("None".to_string(), Vec::new());
        m.insert(F1State::Albums, hm);

        let mut hm: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        hm.insert("None".to_string(), Vec::new());
        m.insert(F1State::Artists, hm);

        let mut hm: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        hm.insert("All".to_string(), Vec::new());
        m.insert(F1State::All, hm);

        TrackDB {
            track_filter_cache: m,
            trackmap: BTreeMap::new(),
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
        debug!("DB after download : {:#?}", self);
    }

    fn add_track_to_filter_cache(&mut self, track_info: &TrackInfo) {
        let new_track_id = track_info.id;
        let playlist = track_info.playlist.clone();
        let artists = track_info.artists.clone();
        let album = track_info.album.clone();

        // update playlist_map
        let playlist_map = self.track_filter_cache.get_mut(&F1State::Playlists).unwrap();
        if playlist_map.contains_key(&playlist) {
            playlist_map.get_mut(&playlist).unwrap().push(new_track_id);
        } else {
            playlist_map.insert(playlist, vec![new_track_id]);
        }

        // update artist_map
        let artist_map = self.track_filter_cache.get_mut(&F1State::Artists).unwrap();
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
        let album_map = self.track_filter_cache.get_mut(&F1State::Albums).unwrap();
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

        // update all map
        let all_map = self.track_filter_cache.get_mut(&F1State::All).unwrap().get_mut("All").unwrap();
        all_map.push(new_track_id);
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
        self.add_track_to_filter_cache(&t_info);
        self.trackmap.insert(new_track_id, t_info);

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

        debug!("Updated filter cache! : {:?}", self.track_filter_cache);
    }

    pub fn remove_track(&mut self, track_id: u32, save: Option<bool>) {
        let t_info = self.trackmap.remove(&track_id).unwrap();
        let t_id = t_info.id;

        self.remove_track_from_filter_cache(&t_info);
        remove_file(t_info.get_file_path()).unwrap();
        if save.unwrap_or(true) {
            self.save_to_file();
        }
        debug!("DB after removal : {:#?}", self);
    }

    fn remove_track_from_filter_cache(&mut self, t_info: &TrackInfo) {
        let t_id = t_info.id;
        let playlist_map = self.track_filter_cache.get_mut(&F1State::Playlists).unwrap();
        let p = playlist_map.get_mut(&t_info.playlist).unwrap();
        p.retain(|&x| x != t_id);

        let all_map = self.track_filter_cache.get_mut(&F1State::All).unwrap();
        let p = all_map.get_mut("All").unwrap();
        p.retain(|&x| x != t_id);

        let album_map = self.track_filter_cache.get_mut(&F1State::Albums).unwrap();
        let p = album_map.get_mut(&t_info.album.clone().unwrap_or("None".to_string())).unwrap();
        p.retain(|&x| x != t_id);

        if p.len() == 0 {
            if let Some(a) = &t_info.album.clone() {
                album_map.remove(a);
            }
        }

        let artist_map = self.track_filter_cache.get_mut(&F1State::Artists).unwrap();
        match t_info.artists.clone() {
            Some(ar) => {
                for a in ar.iter() {
                    let p = artist_map.get_mut(a).unwrap();
                    p.retain(|&x| x != t_id);

                    if p.len() == 0 {
                        artist_map.remove(a);
                    }
                }
            },
            None => {
                let p = artist_map.get_mut(&"None".to_string()).unwrap();
                p.retain(|&x| x != t_id);
            }
        }
    }

    // new_trackinfo.duration can be whatever, since this cannot be edited on the file
    pub fn edit_track(&mut self, mut new_trackinfo: TrackInfo) {
        let prev_trackinfo = self.trackmap.remove(&new_trackinfo.id).unwrap();
        self.remove_track_from_filter_cache(&prev_trackinfo);
        self.add_track_to_filter_cache(&new_trackinfo);
        std::fs::rename(prev_trackinfo.get_file_path(), new_trackinfo.get_file_path()).unwrap();

        new_trackinfo.duration = prev_trackinfo.duration;
        self.trackmap.insert(new_trackinfo.id, new_trackinfo);
        self.save_to_file();
        debug!("DB after edit : {:#?}", self);
    }

    pub fn change_playlist(&mut self, track_id: u32, new_playlist: String) {
        let mut t_info = self.trackmap.get(&track_id).unwrap().clone();
        t_info.playlist = new_playlist;
        self.edit_track(t_info);
    }

    pub fn change_title(&mut self, track_id: u32, new_title: String) {
        let mut t_info = self.trackmap.get(&track_id).unwrap().clone();
        t_info.name = new_title;
        self.edit_track(t_info);
    }

    pub fn change_artists(&mut self, track_id: u32, new_artists: Option<Vec<String>>) {
        let mut t_info = self.trackmap.get(&track_id).unwrap().clone();
        t_info.artists = new_artists;
        self.edit_track(t_info);
    }

    pub fn change_album(&mut self, track_id: u32, new_album: Option<String>) {
        let mut t_info = self.trackmap.get(&track_id).unwrap().clone();
        t_info.album = new_album;
        self.edit_track(t_info);
    }

    pub fn remove_playlist(&mut self, playlist_name: &String) {
        if playlist_name == "Liked" {
            return;
        }

        let tracks = match self.track_filter_cache.get(&F1State::Playlists).unwrap().get(playlist_name) {
            Some(v) => v.clone(),
            None => return
        };

        for id in tracks.iter() {
            self.remove_track(*id, Some(false));
        }
        self.track_filter_cache.get_mut(&F1State::Playlists).unwrap().remove(playlist_name);
        self.save_to_file();
    }

    pub fn create_playlist(&mut self, playlist_name: String) {
        let pmap = self.track_filter_cache.get_mut(&F1State::Playlists).unwrap();
        let existing_playlists = pmap.keys().map(|x| x.clone()).collect::<Vec<String>>();
        if existing_playlists.contains(&playlist_name) {
            return
        }

        pmap.insert(playlist_name, Vec::new());
        self.save_to_file();
    }
}
