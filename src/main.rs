use std::env::{self};
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for: {}, In file: {}",
        config.query, config.file_path
    );

    let contents = match fs::read_to_string(config.file_path) {
        Ok(contents) => contents,
        Err(error) => {
            panic!("Problem reading the file: {:?}", error);
        }
    };
    println!("Contents: {contents}")
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
