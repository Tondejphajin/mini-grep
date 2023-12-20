use std::env::{self};
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

    println!("Searching for: {}, In file: {}", config.query, config.file_path);

    let contents = match fs::read_to_string(config.file_path) {
        Ok(contents) => contents,
        Err(error) => {
            panic!("Problem reading the file: {:?}", error);
        }
    };
    println!("Contents: {contents}")
}

fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config{query,file_path}
} 