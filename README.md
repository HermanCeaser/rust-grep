# MiniGrep

MiniGrep is a simple implementation of a command-line text search tool in Rust. It allows you to search for specific text within a file, with optional case-insensitive searching controlled by an environment variable.

## Features

- Case-sensitive search: Search for text exactly as entered.
- Case-insensitive search: Enabled by setting the environment variable IGNORE_CASE.
- Simple and efficient: Handles command-line arguments and file reading with error handling.

## Installation
Ensure you have Rust installed on your system.
### Clone the repository:
```sh
git clone https://github.com/HermanCeaser/rust-grep
cd rust-grep
```

### Build the project:
```sh
cargo build
```

### Usage
Run the program using the following format:
```sh
cargo run -- <search_term> <file_path>
```
Arguments:

    <search_term>: The term or phrase you want to search for.
    <file_path>: The path to the file where the search will be performed.

Example:
```sh
cargo run -- "Rust" sample.txt
```

*Case-Insensitive Search:*

To enable case-insensitive search, set the IGNORE_CASE environment variable:

`IGNORE_CASE=1 cargo run -- "rust" sample.txt`

Running Tests

This project includes unit tests to ensure functionality. To run the tests:
```sh
cargo test
```
```
Example Output
Case-Sensitive Search:

File sample.txt contents:

Rust:
safe, fast, productive.
Pick three.
Trust me.

Command:

cargo run -- "Rust" sample.txt

Output:

Rust:

Case-Insensitive Search:

Command:

IGNORE_CASE=1 cargo run -- "rust" sample.txt

Output:

Rust:
Trust me.
```

## Project Structure

- main.rs: Handles command-line arguments and orchestrates the program execution.
- lib.rs: Contains the core logic for parsing, searching, and testing.

## Contributing
Contributions, bug reports, and feature requests are welcome! Feel free to open an issue or submit a pull request.
