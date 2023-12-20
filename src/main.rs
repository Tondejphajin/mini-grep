use std::env::{self};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: {query}, In file: {file_path}");

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            panic!("Problem reading the file: {:?}", error);
        }
    };
    println!("Contents: {contents}")

}