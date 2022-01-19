use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when reading the file");
    println!("With text: {}", contents);
}
