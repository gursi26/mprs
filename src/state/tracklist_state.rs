use crate::utils::duration_to_str;

pub struct TracklistState {
    pub items: Vec<TracklistItem>,
}

#[derive(Clone)]
pub struct TracklistItem {
    pub id: u32,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration: String,
}

impl Default for TracklistState {
    fn default() -> Self {
        TracklistState { items: Vec::new() }
    }
}

impl TracklistState {
    pub fn empty(&mut self) {
        self.items.clear();
    }

    pub fn add_item(
        &mut self,
        id: u32,
        name: String,
        artists: Vec<String>,
        album: String,
        duration: u32,
    ) {
        self.items.push(TracklistItem {
            id, name, album,
            artist: artists.join(", "),
            duration: duration_to_str(duration)
        });
    }
}
