use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let (query, file_path) = parse_config(&args);

    println!("Searching for '{query}'");
    println!("in file '{file_path}'");

    let contents = fs::read_to_string(file_path).expect("Should be able to read from file!");
    println!("With text:\n--------------------------\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}