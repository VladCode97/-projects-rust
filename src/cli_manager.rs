use std::io;

pub fn cli_input() -> String {
    println!("Start to write");
    let mut input = String::from("");
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    return input;
}