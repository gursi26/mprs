use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MusicPlayerArgs {
    #[clap(subcommand)]
    pub action_type: ActionType
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Adds a song to a specified playlist (or to "liked" if no playlist specified)
    Add(AddArgs), 
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// The search term for the song to be added
    #[arg(short = 'q', long = "query-term")]
    pub query_term: String,

    /// The playlist to add the song to
    #[arg(short = 'p', long = "playlist", default_value = "liked")]
    pub playlist: String,

    /// The number of search results to display
    #[arg(short = 'c', long = "count")]
    pub count: i8
}

fn main() {
    let args = MusicPlayerArgs::parse();
    let addArgs = match args.action_type {
        ActionType::Add(ref add_args) => add_args
    };
    println!("{:?}", addArgs.query_term);
}
