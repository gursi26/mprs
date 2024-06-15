use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Serialize, Deserialize, Clone)]
pub enum F1State {
    All,
    Playlists,
    Artists,
    Albums,
}
