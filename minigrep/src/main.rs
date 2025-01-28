use std::env;
use std::fs;

pub fn parse_args(args: &[String]) -> (&str, &str) {
    let query = args.get(1).expect("Error: the query value is missing!");
    let file_path = args.get(2).expect("Error: the file_path is missing!");

    (query, file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_args(&args);

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should be able to read file");

    println!("With text:\n {contents}");
}

