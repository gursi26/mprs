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
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// The search term for the song to be added
    #[arg()]
    pub query_term: String,

    /// The playlist to add the song to
    #[arg(short = 'p', long = "playlist", default_value = "liked")]
    pub playlist: String,

    /// The number of search results to display
    #[arg(short = 'c', long = "count", default_value_t = 3)]
    pub count: i8,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// The search term for the song to be removed (Optional, lists all songs in playlist if
    /// unspecified)
    #[arg(short = 'q', long = "query-term", default_value = None)]
    pub query_term: Option<String>,

    /// The playlist from which the song is to be removed
    #[arg(short = 'p', long = "playlist", default_value = "liked")]
    pub playlist: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of new playlist to be created
    #[arg(short = 'n', long = "name")]
    pub playlist_name: String,
}

// -s controls whether song ordering is shuffled or not
// If -s is used without -p, searches all downloaded songs for s, returns top 5
// If -p is used without -s, plays entire playlist
// If both -p and -s are used, returns top 5 matches for s in p, then proceeds to play rest of p
#[derive(Debug, Args)]
pub struct PlayArgs {
    /// The playlist to be played
    #[arg(short = 'p', long = "playlist", default_value = None)]
    pub playlist: Option<String>,

    /// If specified, plays the first song that matches the query_term, and then the rest of the
    /// playlist
    #[arg(short = 'q', long = "query-term", default_value = None)]
    pub query_term: Option<String>,

    /// Include this flag to shuffle play order
    #[arg(short = 's', long = "shuffle", action = ArgAction::SetTrue)]
    pub shuffle: bool,
}
