use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
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
    pub directions: Vec<Direction>,
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
    pub fn get_adventure_map(&mut self) -> Result<AdventureMap, &'static str> {
        match self
            .rooms
            .iter()
            .find(|room| room.name == self.starting_room)
        {
            Some(_) => {}
            None => return Err("Map does not contain starting room in rooms"),
        };

        match self.rooms.iter().find(|room| room.name == self.ending_room) {
            Some(_) => {}
            None => return Err("Map does not contain ending room in rooms"),
        };

        let mut room_name_map = HashMap::new();
        for room in &self.rooms {
            let adventure_room = AdventureRoom {
                name: room.name.clone(),
                description: room.description.clone(),
                items: room.items.as_ref().unwrap_or(Vec::new().as_ref()).clone(),
                directions: room.directions.clone(),
            };
            room_name_map.insert(room.name.clone(), adventure_room);
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

/// `AdventureEngine` exposes the interface for creating and running an adventure game.
pub struct AdventureEngine {
    pub map: AdventureMap,
    pub player: Player,
    pub current_room: String,
}

impl AdventureEngine {
    pub fn new(mut map_data: MapData) -> Result<AdventureEngine, &'static str> {
        let adventure_map = match map_data.get_adventure_map() {
            Ok(map) => map,
            Err(e) => return Err(e),
        };

        Ok(AdventureEngine {
            current_room: adventure_map.starting_room.clone(),
            map: adventure_map,
            player: Player::new(Vec::new()),
        })
    }
}
