use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Direction {
    direction_name: String,
    destination: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Room {
    name: String,
    description: String,
    items: Option<Vec<String>>,
    directions: Vec<Direction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    starting_room: String,
    ending_room: String,
    rooms: Vec<Room>,
}

pub struct AdventureMap {
    starting_room: String,
    ending_room: String,
    room_name_map: HashMap<String, Room>,
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
            room_name_map.insert(room.name.clone(), room.clone());
        }

        let am = AdventureMap {
            starting_room: self.starting_room.clone(),
            ending_room: self.ending_room.clone(),
            room_name_map,
        };

        Ok(am)
    }
}

struct Player {
    items: Vec<String>,
}

impl Player {
    pub fn new(initial_items: Vec<String>) -> Player {
        Player {
            items: initial_items,
        }
    }
}

pub struct AdventureEngine {
    map: AdventureMap,
    player: Player,
    current_room: String,
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
