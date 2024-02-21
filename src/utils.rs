use std::path::PathBuf;
use dirs::home_dir;
use prettytable::{Table, Cell, Row};

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
