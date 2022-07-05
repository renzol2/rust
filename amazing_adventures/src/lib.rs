use clap::Parser;
use std::fs;
use std::io;
use std::process;

mod engine;
use engine::*;

#[derive(Parser, Debug)]
pub struct Args {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub fn run_game(args: Args) {
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

    let mut adventure_engine: AdventureEngine = match AdventureEngine::new(map) {
        Ok(engine) => engine,
        Err(msg) => {
            eprintln!("An error occurred while initializing adventure: {}", msg);
            process::exit(1);
        }
    };

    // Game loop begins
    let final_state: AdventureState = initiate_game_loop(&mut adventure_engine);

    match final_state {
        AdventureState::Finish => {
            println!(
                "{}",
                adventure_engine
                    .map
                    .room_name_map
                    .get(&adventure_engine.current_room)
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

pub fn initiate_game_loop(engine: &mut AdventureEngine) -> AdventureState {
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

        let command = process_input(input, false);

        let state = engine.process_command(&command);

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
fn process_input(input: String, is_case_sensitive: bool) -> Command {
    // TODO: add support for items
    // Separate commands from directions/items
    let input = input.trim();
    let tokens: Vec<&str> = input.splitn(2, " ").collect();
    let command = tokens[0];
    let command: String = if is_case_sensitive {
        command.to_string()
    } else {
        command.to_lowercase()
    };

    // Identify command
    if tokens.len() == 2 {
        let object = tokens[1].trim().to_string();
        return match command.as_str() {
            "go" => Command::Go { direction: object },
            "take" => Command::Take { item: object },
            "drop" => Command::Drop { item: object },
            _ => Command::Invalid,
        };
    } else if command == "quit" {
        return Command::Quit;
    } else {
        return Command::Invalid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_command_empty_string_is_invalid() {
        let input = String::from("");
        let processed_input = process_input(input, true);
        let expected = Command::Invalid;
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_lowercase() {
        let input = String::from("go north");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "north".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_uppercase() {
        let input = String::from("go SOUTH");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "SOUTH".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_mixed_case() {
        let input = String::from("go wEsT");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "wEsT".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_lowercase_spaced() {
        let input = String::from("    go     up    ");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "up".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_uppercase_spaced() {
        let input = String::from("    go     UP    ");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "UP".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_single_word_mixed_case_spaced() {
        let input = String::from("    go           down the spooky hallway    ");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "down the spooky hallway".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_multi_word_lowercase() {
        let input = String::from("go down the hallway");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "down the hallway".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_multi_word_uppercase() {
        let input = String::from("go FAR, FAR AWAY");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "FAR, FAR AWAY".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_multi_word_mixed_case() {
        let input = String::from("go behind the Holy Cathedral");
        let processed_input = process_input(input, true);
        let expected = Command::Go {
            direction: "behind the Holy Cathedral".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_go_without_direction_is_invalid() {
        let input = String::from("go ");
        let processed_input = process_input(input, true);
        let expected = Command::Invalid;
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_quit_case_sensitive_valid() {
        let input = String::from("quit");
        let processed_input = process_input(input, true);
        let expected = Command::Quit;
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_quit_case_sensitive_invalid() {
        let input = String::from("quIT");
        let processed_input = process_input(input, true);
        let expected = Command::Invalid;
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_quit_case_insensitive_valid() {
        let input = String::from("quIT");
        let processed_input = process_input(input, false);
        let expected = Command::Quit;
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_without_item_is_invalid() {
        let input = String::from("take");
        let processed_input = process_input(input, false);
        let expected = Command::Invalid;
        assert_eq!(processed_input, expected);
    }

    // Take
    #[test]
    fn process_command_take_single_word_lowercase() {
        let input = String::from("take apple");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "apple".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_single_word_uppercase() {
        let input = String::from("take APPLE");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "APPLE".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_single_word_mixed_case() {
        let input = String::from("take oRaNgE");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "oRaNgE".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_single_word_lowercase_spaced() {
        let input = String::from("    take     tea    ");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "tea".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_single_word_uppercase_spaced() {
        let input = String::from("    take     COFFEE    ");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "COFFEE".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_single_word_mixed_case_spaced() {
        let input = String::from("    take           Excalibur    ");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "Excalibur".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_multi_word_lowercase() {
        let input = String::from("take rotten flesh");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "rotten flesh".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_multi_word_uppercase() {
        let input = String::from("take BUTTON OF DOOM");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "BUTTON OF DOOM".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_multi_word_mixed_case() {
        let input = String::from("take Renzo's Nintendo Switch");
        let processed_input = process_input(input, true);
        let expected = Command::Take {
            item: "Renzo's Nintendo Switch".to_string(),
        };
        assert_eq!(processed_input, expected);
    }

    #[test]
    fn process_command_take_without_direction_is_invalid() {
        let input = String::from("take ");
        let processed_input = process_input(input, true);
        let expected = Command::Invalid;
        assert_eq!(processed_input, expected);
    }
}
