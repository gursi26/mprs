pub mod utils;

// TODO: define get_track_info in utils.rs
//          - returns TrackInfo struct
//          - distinct from get_track_information currently used

struct TrackDB {
    db_file_path: &Path,  // path to database file
    track_map: HashMap<&'static str, HashMap<&'static str, Vec<u64>>>,
    track_list: Vec<TrackInfo>,  // indices in this list are track ids
    max_id: u64,  // max_id is the next id to be assigned (i.e. one more than the last id assigned)
}

struct TrackInfo {
    id: u64,
    name: &'static str,
    artist: &'static str,
    album: &'static str,
    playlist: &'static str,
    duration: u64,
}

impl TrackInfo {
    fn new(id: u64, name: &'static str, artist: &'static str, album: &'static str, playlist: &'static str, duration: u64) -> Self {
        Self {
            id,
            name,
            artist,
            album,
            playlist,
            duration,
        }
    }
}

impl TrackDB {
    fn new() -> Self {
        Self {
            db_file_path: utils::config_path(),
            track_map: HashMap::new(),
            track_list: Vec::new(),
            max_id: 0,
        };
        track_map.insert("Albums", HashMap::new());
        track_map.insert("Artists", HashMap::new());
        track_map.insert("Playlists", HashMap::new());
    }

    fn add_track_to_db(&mut self, path_to_track: &Path) -> u64 {
        let id = self.max_id;  // assign track id
        self.max_id += 1;  // increment max_id to reflect addition of new track
        let track_info = utils::get_track_info(path_to_track);  // struct TrackInfo datatype

        self.track_list.push(track_info);  // add track to track_list
        self.track_map["Albums"].entry(track_info.album).or_insert(Vec::new()).push(id);  // add track's album to track_map
        self.track_map["Artists"].entry(track_info.artist).or_insert(Vec::new()).push(id);  // add track's artist to track_map
        self.track_map["Playlists"].entry(track_info.playlist).or_insert(Vec::new()).push(id);  // add track's playlist to track_map
        id
    }

    // fn add_track(&mut self, name: &'static str, artist: &'static str, album: &'static str, playlist: &'static str, duration: u64) -> u64 {
    //     let id = self.max_id;
    //     self.max_id += 1;
    //     self.track_list.push(TrackInfo::new(id, name, artist, album, playlist, duration));
    //     self.track_map.entry(playlist).or_insert(HashMap::new()).entry(artist).or_insert(Vec::new()).push(id);
    //     id
    // }

    // fn get_tracks(&self, playlist: &'static str, artist: &'static str) -> Vec<&TrackInfo> {
    //     self.track_map.get(playlist).and_then(|m| m.get(artist)).map(|ids| ids.iter().map(|&id| &self.track_list[id as usize]).collect()).unwrap_or(Vec::new())
    // }
}
