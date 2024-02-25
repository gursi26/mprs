use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MusicPlayerArgs {
    #[clap(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Adds a song to a specified playlist (or to "liked" if no playlist specified)
    Add(AddArgs),

    /// Removed a song from a specified playlist (or "liked" if no playlist specified)
    Remove(RemoveArgs),

    /// Creates a new playlist
    Create(CreateArgs),

    /// Plays a specific playlist (or "liked" if no playlist specified)
    Play(PlayArgs),

    /// Lists songs within a playlist or all playlists
    List(ListArgs),

    /// Moves/Copies a song from one playlist to another
    Move(MoveArgs),

    /// Opens the directory where audio files are stored
    Open
}

#[derive(Debug, Args)]
pub struct MoveArgs {
    /// The name of the track to be copied (returns a selection table if multiple matches found)
    #[arg()]
    pub track: String,

    /// The source playlist (looks for source track in all songs if unspecified)
    #[arg(short = 'p', long = "playlist", default_value = None)]
    pub source_playlist: Option<String>,

    /// Destination playlist where track is to be moved/copied
    #[arg(short = 'd', long = "dest")]
    pub dest_playlist: String,

    /// Flag for copy instead of move
    #[arg(short = 'c', long = "copy", action = ArgAction::SetTrue)]
    pub copy: bool,
}


#[derive(Debug, Args)]
pub struct ListArgs {
    /// The term for the playlist to be listed (lists playlists if not provided)
    #[arg(short = 'p', long = "playlist", default_value = None)]
    pub playlist: Option<String>,

    /// Lists all songs in all playlists
    #[arg(short = 'a', long = "all", action = ArgAction::SetTrue)]
    pub list_all: bool,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// The search term for the song to be added
    #[arg()]
    pub query_term: String,

    /// The playlist to add the song to
    #[arg(short = 'p', long = "playlist", default_value = "liked")]
    pub playlist: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// The search term for the track to be removed (Optional, lists all tracks in playlist if
    /// unspecified)
    #[arg(default_value = None)]
    pub track: Option<String>,

    /// The playlist from which the song is to be removed (Optional, searches in all playlists if
    /// unspecified)
    #[arg(short = 'p', long = "playlist", default_value = None)]
    pub playlist: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of new playlist to be created
    #[arg(short = 'n', long = "name")]
    pub playlist_name: String,
}

// -s controls whether song ordering is shuffled or not
// If -q is used without -p, searches all downloaded songs for s, returns top 5
// If -p is used without -q, plays entire playlist
// If both -p and -q are used, returns top 5 matches for q in p, then proceeds to play rest of p
#[derive(Debug, Args)]
pub struct PlayArgs {
    /// If specified, plays the first song that matches the query_term, and then the rest of the
    /// playlist
    #[arg(default_value = None)]
    pub query_term: Option<String>,

    /// The playlist to be played
    #[arg(short = 'p', long = "playlist", default_value = None)]
    pub playlist: Option<String>,

    /// Include this flag to shuffle play order
    #[arg(short = 's', long = "shuffle", action = ArgAction::SetTrue)]
    pub shuffle: bool,
}
