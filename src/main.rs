use std::env;
use std::process;

use mini_grep::Config;

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

    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
