use std::path::PathBuf;
use dirs::home_dir;
use prettytable::{Table, Cell, Row};
use std::fs::read_dir;


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

