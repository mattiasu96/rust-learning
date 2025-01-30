use core::panic;
use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough input arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{query, file_path})

    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    let config = match config {
        Ok(config) => config,
        Err(error) => panic!("Config not valid: {error:?}")
    };

    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read file");

    println!("With text:\n {contents}");
}

