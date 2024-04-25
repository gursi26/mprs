use std::path::PathBuf;

// TODO: Change to queue to hold track id and play lookup in db
pub struct TrackQueue {
    pub queue: Vec<PathBuf>,
    pub curr_idx: i32,
}

impl TrackQueue {
    pub fn new() -> Self {
        TrackQueue {
            queue: Vec::new(),
            curr_idx: -1
        }
    }

    pub fn append(&mut self, p: PathBuf) {
        self.queue.push(p);
    }

    pub fn prepend(&mut self, p: PathBuf) {
        self.queue.insert(1, p);
    }

    pub fn next_track(&mut self) {
        if self.curr_idx < (self.queue.len() - 1) as i32 {
            self.curr_idx += 1;
        }
    }

    pub fn prev_track(&mut self) {
        if self.curr_idx > 0 {
            self.curr_idx -= 1;
        }
    }

    pub fn get_curr_track_path(&self) -> &PathBuf{
        self.queue.get(self.curr_idx as usize).unwrap()
    }
}
