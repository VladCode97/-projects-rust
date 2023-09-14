use std::fs::File;
use std::io::{BufWriter, Write};

mod errors_enum;
pub fn create_file(name_file: &String) -> File { File::create(name_file).ok().unwrap() }

pub fn write_file(file: &mut File, content: String) {
    let mut buffer_write = BufWriter::new(file);
    buffer_write.write_all(content.as_bytes()).expect();
    buffer_write.flush().expect("Error writing file");
}
