use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = parse_config(&args);

    println!("Searching for '{}'", config.query);
    println!("in file '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read from file!");
    println!("With text:\n--------------------------\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path}
}