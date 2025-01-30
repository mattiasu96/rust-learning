use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {

    fn new(args: &[String]) -> Option<Config> {
        if args.len() < 3 {
            panic!("Not enough input arguments")
        }
        let query = args.get(1)?.clone();
        let file_path = args.get(2)?.clone();

        Some(Config{query, file_path})

    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let config = match config {
        Some(config) => config,
        None => panic!("Non valid config!")
        
    };

    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read file");

    println!("With text:\n {contents}");
}

