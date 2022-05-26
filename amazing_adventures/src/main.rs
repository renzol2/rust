use amazing_adventures::{AdventureEngine, Command, MapData};
use clap::Parser;
use std::fs;
use std::io;
use std::process;

#[derive(Parser, Debug)]
struct Args {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.path).unwrap_or_else(|err| {
        eprintln!(
            "An error occurred while locating JSON file: {}",
            err.to_string()
        );
        process::exit(1);
    });
    let map: MapData = serde_json::from_str(&file).unwrap_or_else(|err| {
        eprintln!(
            "An error occurred while deserializing JSON file: {}",
            err.to_string()
        );
        process::exit(1);
    });

    let mut engine = match AdventureEngine::new(map) {
        Ok(engine) => engine,
        Err(msg) => {
            eprintln!("An error occurred while initializing adventure: {}", msg);
            process::exit(1);
        }
    };

    loop {
        let current_room = engine.map.room_name_map.get(&engine.current_room).unwrap();
        println!("{}", current_room.description);
        println!("You can go in these directions: ");
        for direction in &current_room.directions_map {
            println!("- {}", direction.0);
        }
        println!("\nWhat would you like to do?\n");

        // println!("You see these items on the ground: ");
        // for item in &current_room.items {
        //     println!("- {}", item);
        // }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // FIXME: hardcoded to process as directions, need to make function to convert
        engine.process_command(Command::Go {
            direction: String::from(input.trim()),
        });
        println!("");
    }
}
