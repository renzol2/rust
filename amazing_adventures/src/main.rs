use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Direction {
    direction_name: String,
    room: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Room {
    name: String,
    description: String,
    items: Option<Vec<String>>,
    directions: Vec<Direction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Adventure {
    starting_room: String,
    ending_room: String,
    rooms: Vec<Room>,
}

#[derive(Parser, Debug)]
struct Args {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.path).expect("Error while reading file contents");
    let a: Adventure = serde_json::from_str(&file).expect("Error while deserializing JSON file");

    println!("{} {}", a.starting_room, a.ending_room);
}
