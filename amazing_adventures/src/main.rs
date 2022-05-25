use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs, str::FromStr};

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

fn main() {
    parse_adventure_json("resources/dorm.json");
}

fn parse_adventure_json(filename: &str) {
    let file = fs::read_to_string(filename).expect("Something went wrong");
    let a: Adventure = serde_json::from_str(&file).expect("Something went wrong");

    println!("{} {}", a.starting_room, a.ending_room);
}
