use amazing_adventures::{AdventureEngine, MapData};
use clap::Parser;
use std::fs;
use std::io;

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
        let current_room = engine.map.room_name_map.get(&engine.current_room).unwrap();
        println!("{}", current_room.description);
        println!("You see these items on the ground: ");
        for item in &current_room.items {
            println!("- {}", item);
        }

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
    }
}
