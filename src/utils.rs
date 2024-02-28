use dirs::home_dir;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
// use prettytable::{Table, Cell, Row};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use lofty::{read_from_path, Accessor, AudioFile, Probe, Tag, TagExt, TaggedFileExt};
use std::fs::read_dir;

pub enum UserInput {
    Quit,
    Pause,
    Next,
    Previous,
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
    ResetSpeed,
    DoNothing,
}

pub fn config_path() -> PathBuf {
    let mut config_path = home_dir().unwrap();
    config_path.push(".config");
    config_path.push("mprs");
    config_path.push("config.yaml");
    config_path
}

// FIX: Use base_dir from config.yaml
pub fn base_dir() -> PathBuf {
    let mut base_dir = home_dir().unwrap();
    base_dir.push("mprs-music");
    base_dir
}

pub fn list_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = read_dir(path).unwrap().map(|i| i.unwrap().path()).collect();
    files.retain(|x| x.as_path().file_name().unwrap().to_str().unwrap() != ".DS_Store");
    files
}

pub fn get_track_information(track_path: &PathBuf) -> Vec<String> {
    let name = get_track_name(&track_path);
    let artist = get_artist(&track_path);
    let playlist = get_playlist(&track_path);
    let duration = get_duration(&track_path).to_string();
    let date_added = get_date_added(&track_path);
    vec![name, artist, playlist, duration, date_added]
}

pub fn get_track_name(track_path: &PathBuf) -> String {
    track_path
        .as_path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn print_table(table_content: &Vec<Vec<String>>) {
    let size = termsize::get().unwrap();
    let mut table = Table::new();

    let mut row_vec: Vec<Cell>;
    // Insert all other rows
    for (i, x) in table_content.iter().enumerate() {
        row_vec = x.iter().map(|i| Cell::new(&(*i)[..])).collect();
        if i == 0 {
            row_vec.insert(0, Cell::new("#"));
            table.set_header(row_vec);
        } else {
            row_vec.insert(0, Cell::new(&(i).to_string()[..]));
            table.add_row(row_vec);
        }
    }

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(size.cols);

    println!("{table}");
}

pub fn get_input_key() -> UserInput {
    if crossterm::event::poll(std::time::Duration::from_millis(250)).unwrap() {
        // If a key event occurs, handle it
        if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => UserInput::Quit,
                    crossterm::event::KeyCode::Char('n') => UserInput::Next,
                    crossterm::event::KeyCode::Char('b') => UserInput::Previous,
                    crossterm::event::KeyCode::Char('p') => UserInput::Pause,
                    crossterm::event::KeyCode::Char('+') => UserInput::VolumeUp,
                    crossterm::event::KeyCode::Char('-') => UserInput::VolumeDown,
                    crossterm::event::KeyCode::Right => UserInput::SpeedUp,
                    crossterm::event::KeyCode::Left => UserInput::SpeedDown,
                    crossterm::event::KeyCode::Up => UserInput::ResetSpeed,
                    _ => UserInput::DoNothing,
                }
            } else {
                UserInput::DoNothing
            }
        } else {
            UserInput::DoNothing
        }
    } else {
        UserInput::DoNothing
    }
}

pub fn get_instruction_string() -> String {
    format!(" Quit: q | Pause/Play: p | Next: n | Previous: b | Increase/Decrease Volume: +/- | Increase/Decrease Speed: →/← | Reset Speed: ↑ ")
}

pub fn get_duration(path: &PathBuf) -> u64 {
    let duration = read_from_path(path.clone())
        .unwrap()
        .properties()
        .duration();
    duration.as_secs()
}

pub fn set_artist(path: &PathBuf, artist: &String) {
    let mut tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag,
        None => {
            if let Some(first_tag) = tagged_file.first_tag_mut() {
                first_tag
            } else {
                let tag_type = tagged_file.primary_tag_type();

                eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");
                tagged_file.insert_tag(Tag::new(tag_type));

                tagged_file.primary_tag_mut().unwrap()
            }
        }
    };

    tag.set_artist(artist.clone());
    tag.save_to_path(path).unwrap();
}

pub fn get_playlist(track_path: &PathBuf) -> String {
    track_path
        .as_path()
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_date_added(track_path: &PathBuf) -> String {
    let metadata = std::fs::metadata(track_path).unwrap();
    if let Ok(time) = metadata.created() {
        let datetime: DateTime<Utc> = time.clone().into();
        datetime.to_string()
    } else {
        String::from("")
    }
}

pub fn get_artist(path: &PathBuf) -> String {
    if !path.is_file() {
        panic!("ERROR: Path is not a file!");
    }

    let tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    tag.artist().as_deref().unwrap_or("Unknown").to_string()
}
