use std::path::PathBuf;
use dirs::home_dir;
use prettytable::{Table, Cell, Row};
use std::fs::read_dir;
use lofty::{read_from_path, AudioFile};

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

pub fn base_dir() -> PathBuf {
    let mut base_dir = home_dir().unwrap();
    base_dir.push("mprs-music");
    base_dir
}

pub fn list_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = read_dir(path)
        .unwrap()
        .map(|i| i.unwrap().path())
        .collect();
    files.retain(|x| x.as_path().file_name().unwrap().to_str().unwrap() != ".DS_Store");
    files
}

pub fn print_table(table_content: &Vec<Vec<String>>) {
    let mut table = Table::new();

    let mut row_vec: Vec<Cell>;
    // Insert all other rows
    for (i, x) in table_content.iter().enumerate() {
        row_vec = x.iter().map(|i| Cell::new(&(*i)[..])).collect();
        if i == 0 {
            row_vec.insert(0, Cell::new("#"));
        } else {
            row_vec.insert(0, Cell::new(&(i).to_string()[..]));
        }
        table.add_row(Row::new(row_vec));
    }
    table.printstd();
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
    format!("Quit: q | Pause/Play: p | Next: n | Previous: b | Increase/Decrease Volume: +/- | Increase/Decrease Speed: →/← | Reset Speed: ↑ ")
}

pub fn get_duration(path: &PathBuf) -> u64 {
    let duration = read_from_path(path.clone()).unwrap().properties().duration();
    duration.as_secs()
}
