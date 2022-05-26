use clap::Parser;
use std::fs;
use std::io;
use amazing_adventures::{MapData, AdventureEngine};

#[derive(Parser, Debug)]
struct Args {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.path).expect("Error while reading file contents");
    let map: MapData = serde_json::from_str(&file).expect("Error while deserializing JSON file");

    let engine = match AdventureEngine::new(map) {
        Ok(engine) => engine,
        Err(msg) => panic!("{}", msg),
    };

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    }
}
