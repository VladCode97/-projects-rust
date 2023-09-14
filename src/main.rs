use std::io;
use file_manager::{create_file, write_file};
use cli_manager::{cli_input};
mod file_manager;
mod cli_manager;
fn main() {
    let name_file: String = String::from("File.txt");
    let mut file = create_file(&name_file);
    let content = cli_input();
    write_file(&mut file, content);
}
