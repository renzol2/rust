use amazing_adventures::{AdventureEngine, AdventureState, Command, MapData};
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

    // Game loop begins
    let final_state = initiate_game_loop(&mut engine);

    match final_state {
        AdventureState::Finish => {
            println!(
                "{}",
                engine
                    .map
                    .room_name_map
                    .get(&engine.current_room)
                    .unwrap()
                    .description
            );
            println!("You won!");
        }
        AdventureState::Quit => {
            println!("You quit.");
        }
        _ => (),
    }
}

fn initiate_game_loop(engine: &mut AdventureEngine) -> AdventureState {
    let final_state = loop {
        let current_room = engine.map.room_name_map.get(&engine.current_room).unwrap();
        println!("\n{}", current_room.description);
        println!("You can go in these directions: ");
        for direction in &current_room.directions_map {
            println!("- {}", direction.0);
        }
        let items = &current_room.items;
        if !items.is_empty() {
            println!("You see these items on the ground: ");
            for item in items {
                println!("- {}", item);
            }
        }
        println!("\nWhat would you like to do?\n");


        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = process_input(input);

        let state = engine.process_command(command);

        match state {
            AdventureState::Success => (),
            AdventureState::Failure { error_msg } => println!("{}", error_msg),
            AdventureState::Finish => break AdventureState::Finish,
            AdventureState::Quit => break AdventureState::Quit,
        }
        println!("");
    };

    final_state
}

/// Converts raw user input into commands.
/// Trims whitespace and return characters.
fn process_input(input: String) -> Command {
    // FIXME: handle multi-word directions, case sensitivity
    if input.contains("go") {
        let tokens: Vec<&str> = input.split(" ").collect();
        let mut direction = tokens[1].to_string();

        // FIXME: there's probably a better way to do this
        direction.pop();  // pop \r
        direction.pop();  // pop \n
        return Command::Go { direction };
    } else if input.contains("quit") {
        return Command::Quit;
    } else {
        return Command::Invalid;
    }
}
