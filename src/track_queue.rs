use std::path::PathBuf;
use rand::thread_rng;
use rand::seq::SliceRandom;

use log::debug;

#[derive(Clone, Debug)]
pub enum TrackType {
    RegQueueTrack(u32),
    ExQueueTrack(u32),
    None
}

#[derive(Debug)]
pub struct TrackQueue {
    pub reg_queue: Vec<u32>,
    pub exp_queue: Vec<u32>, 
    pub curr_track: TrackType,
    pub played_tracks: Vec<u32>
}

impl TrackQueue {
    pub fn new() -> Self {
        TrackQueue {
            reg_queue: Vec::new(),
            exp_queue: Vec::new(),
            curr_track: TrackType::None,
            played_tracks: Vec::new(),
        }
    }

    pub fn shuffle_reg_queue(&mut self) {
        self.reg_queue.shuffle(&mut thread_rng());
    }

    pub fn empty_queue(&mut self) {
        self.reg_queue = Vec::new();
        self.exp_queue = Vec::new();
        self.played_tracks = Vec::new();
    }

    pub fn empty_reg_queue(&mut self) {
        self.reg_queue = Vec::new();
        self.played_tracks = Vec::new();
    }

    pub fn play_next(&mut self, p: u32) {
        self.exp_queue.insert(0, p);
    }

    pub fn add_to_queue(&mut self, p: u32) {
        self.exp_queue.push(p);
    }

    pub fn add_to_reg_queue(&mut self, p: u32) {
        self.reg_queue.push(p);
    }

    pub fn prepend_to_reg_queue(&mut self, p: u32) {
        self.reg_queue.insert(0, p);
    }

    pub fn get_curr_track(&self) -> Option<u32> {
        match self.curr_track.clone() {
            TrackType::RegQueueTrack(t) => Some(t),
            TrackType::ExQueueTrack(t) => Some(t),
            TrackType::None => None
        }
    }

    pub fn next_track(&mut self) {
        // move curr track to played tracks vec
        if let TrackType::RegQueueTrack(t) = self.curr_track.clone() {
            self.played_tracks.push(t);
        }
        self.curr_track = TrackType::None;

        // get next track from explicit queue first, if empty look at regular queue
        // if both empty, reset regular queue to played tracks and call next track again
        if self.exp_queue.len() > 0 {
            self.curr_track = TrackType::ExQueueTrack(self.exp_queue.remove(0));
        } else if self.reg_queue.len() > 0 {
            self.curr_track = TrackType::RegQueueTrack(self.reg_queue.remove(0));
        } else {
            self.reg_queue = self.played_tracks.clone();
            self.played_tracks = Vec::new();
            self.next_track();
        }
        debug!("{:?}", self);
    }

    pub fn prev_track(&mut self) {
        if self.played_tracks.len() > 0 {
            match self.curr_track.clone() {
                TrackType::RegQueueTrack(t) => { self.reg_queue.insert(0, t) },
                TrackType::ExQueueTrack(t) => { self.exp_queue.insert(0, t) },
                TrackType::None => {}
            }
            self.curr_track = TrackType::RegQueueTrack(self.played_tracks.pop().unwrap());
        }
        debug!("{:?}", self);
    }

    pub fn add_ordered_tracklist_to_reg_queue(&mut self, mut tids: Vec<u32>) {
        if let Some(tid) = self.get_curr_track() {
            self.empty_reg_queue();
            let mut before_tracks = Vec::new();
            loop {
                let removed_tid = tids.remove(0);
                if removed_tid == tid {
                    break
                }
                before_tracks.push(removed_tid);
            }

            tids.append(&mut before_tracks);
            for tid in tids.into_iter() {
                self.add_to_reg_queue(tid);
            }
        }
    }
}
