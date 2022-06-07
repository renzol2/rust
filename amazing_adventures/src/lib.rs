use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A `Direction` represents a direction a Player can take.
pub struct Direction {
    direction_name: String,
    destination: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A `RoomData` struct contains room information parsed from JSON.
pub struct RoomData {
    pub name: String,
    pub description: String,
    pub items: Option<Vec<String>>,
    pub directions: Vec<Direction>,
}

/// An `AdventureRoom` represents a room that a Player can enter and exit.
pub struct AdventureRoom {
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
    pub directions_map: HashMap<String, Direction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// `MapData` contains map information parsed from JSON.
pub struct MapData {
    starting_room: String,
    ending_room: String,
    rooms: Vec<RoomData>,
}

impl MapData {
    pub fn get_adventure_map(&self) -> Result<AdventureMap, String> {
        // Check if map has starting room
        match self
            .rooms
            .iter()
            .find(|room| room.name == self.starting_room)
        {
            Some(_) => {}
            None => return Err(String::from("Map does not contain starting room in rooms")),
        };

        // Check if map has ending room room
        match self.rooms.iter().find(|room| room.name == self.ending_room) {
            Some(_) => {}
            None => return Err(String::from("Map does not contain ending room in rooms")),
        };

        // Construct room map from rooms vector
        let mut room_name_map = HashMap::new();
        for room in &self.rooms {
            let directions_map: HashMap<String, Direction> = room
                .directions
                .iter()
                .map(|dir| (dir.direction_name.clone(), dir.clone()))
                .collect();

            let adventure_room = AdventureRoom {
                name: room.name.clone(),
                description: room.description.clone(),
                items: room.items.as_ref().unwrap_or(Vec::new().as_ref()).clone(),
                directions_map,
            };
            room_name_map.insert(room.name.clone(), adventure_room);
        }

        // Check if every direction leads to a room
        for room in &self.rooms {
            for direction in &room.directions {
                if !room_name_map.contains_key(&direction.destination) {
                    let error_msg = format!(
                        "Room {} contains a direction {} to room {}, which does not exist.",
                        room.name, direction.direction_name, direction.destination
                    );
                    return Err(error_msg);
                }
            }
        }

        let am = AdventureMap {
            starting_room: self.starting_room.clone(),
            ending_room: self.ending_room.clone(),
            room_name_map,
        };

        Ok(am)
    }
}

/// `AdventureMap` represents the entire map for an adventure game.
pub struct AdventureMap {
    pub starting_room: String,
    pub ending_room: String,
    pub room_name_map: HashMap<String, AdventureRoom>,
}

/// `Player` holds the player's state during an adventure game.
pub struct Player {
    items: Vec<String>,
}

impl Player {
    pub fn new(initial_items: Vec<String>) -> Player {
        Player {
            items: initial_items,
        }
    }
}

pub enum Command {
    Go { direction: String },
    Take { item: String },
    Drop { item: String },
    Quit,
    Invalid,
}

/// `AdventureEngine` exposes the interface for creating and running an adventure game.
pub struct AdventureEngine {
    pub map: AdventureMap,
    pub player: Player,
    pub current_room: String,
}

impl AdventureEngine {
    /// Constructs a new `AdventureEngine` struct from given map data.
    /// Returns an error if map data is not valid.
    pub fn new(map_data: MapData) -> Result<AdventureEngine, String> {
        let adventure_map = match map_data.get_adventure_map() {
            Ok(map) => map,
            Err(error_msg) => return Err(error_msg),
        };

        Ok(AdventureEngine {
            current_room: adventure_map.starting_room.clone(),
            map: adventure_map,
            player: Player::new(Vec::new()),
        })
    }

    /// Updates game state based on player command. Returns true
    /// if command was received successfully, or false otherwise.
    pub fn process_command(&mut self, command: Command) -> AdventureState {
        match command {
            Command::Go { direction } => {
                let current_room = self
                    .map
                    .room_name_map
                    .get(&self.current_room)
                    .unwrap_or_else(|| panic!("Error: current room not found in rooms map"));

                let destination_room = current_room.directions_map.get(&direction);

                match destination_room {
                    Some(direction) => {
                        self.current_room = direction.destination.clone();
                        if direction.destination == self.map.ending_room {
                            AdventureState::Finish
                        } else {
                            AdventureState::Success
                        }
                    }
                    None => AdventureState::Failure {
                        error_msg: "You can't go that direction from this room.".to_string(),
                    },
                }
            }
            Command::Quit => AdventureState::Quit,
            _ => AdventureState::Failure {
                error_msg: "That command is not recognized.".to_string(),
            },
        }
    }
}

pub enum AdventureState {
    Success,
    Failure { error_msg: String },
    Quit,
    Finish,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn get_adventure_map_creates_map_correctly() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        
        let adventure_map = map.get_adventure_map().unwrap();

        assert_eq!(adventure_map.starting_room, "DormRoom");
        assert_eq!(adventure_map.ending_room, "LaundryRoom");

        // TODO: ensure some rooms have correct names, descriptions, items, and directions
    }
}