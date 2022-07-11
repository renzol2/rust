use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

/// An `AdventureRoom` represents a room that a Player can enter and exit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdventureRoom {
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
    pub directions_map: HashMap<String, Direction>,
}

impl AdventureRoom {
    /// Finds an item of matching name and returns it.
    /// Returns an error if the item is not found.    
    fn take_item(&mut self, item: &String) -> Result<String, &'static str> {
        let index = match self.items.iter().position(|i| i == item) {
            Some(i) => i,
            None => return Err("Item not found"),
        };
        Ok(self.items.remove(index))
    }

    fn drop_item(&mut self, item: String) {
        unimplemented!();
    }

    fn item_exists(&self, item: &String) -> bool {
        unimplemented!();
    }
}

/// `AdventureMap` represents the entire map for an adventure game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdventureMap {
    pub starting_room: String,
    pub ending_room: String,
    pub room_name_map: HashMap<String, AdventureRoom>,
}

/// `Player` holds the player's state during an adventure game.
#[derive(Clone)]
pub struct Player {
    inventory: Vec<String>,
}

impl Player {
    pub fn new(initial_items: Vec<String>) -> Player {
        Player {
            inventory: initial_items,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Go { direction: String },
    Take { item: String },
    Drop { item: String },
    Quit,
    Invalid,
}

/// `AdventureEngine` exposes the interface for creating and running an adventure game.
#[derive(Clone)]
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
            map: adventure_map.clone(),
            player: Player::new(Vec::new()),
        })
    }

    fn get_current_room(&self) -> &AdventureRoom {
        self.map
            .room_name_map
            .get(&self.current_room)
            .unwrap_or_else(|| panic!("Error: current room not found in rooms map"))
    }

    fn get_current_room_mut(&mut self) -> &mut AdventureRoom {
        self.map
            .room_name_map
            .get_mut(&self.current_room)
            .unwrap_or_else(|| panic!("Error: current room not found in rooms map"))
    }

    fn item_in_current_room(&self, item: &String) -> bool {
        self.get_current_room().items.contains(item)
    }

    /// Adds the item parameter to the player's inventory while removing it from current room
    fn take_item_from_current_room(&mut self, item: &String) {
        // Remove item from room
        let found_item = match self.get_current_room_mut().take_item(item) {
            Ok(item) => item,
            Err(_) => panic!("Item not found"), 
        };

        // Put item in player inventory
        self.player.inventory.push(found_item);
    }

    fn drop_item_in_current_room(&mut self, item: &String) {
        // Remove item from player inventory
        // Remove item from room
        ()
    }

    /// Updates game state based on player command. Returns true
    /// if command was received successfully, or false otherwise.
    pub fn process_command(&mut self, command: &Command) -> AdventureState {
        match command {
            Command::Go { direction } => {
                let destination_room = self.get_current_room().directions_map.get(direction);

                match destination_room {
                    Some(direction) => {
                        let destination = direction.destination.clone();
                        self.current_room = destination;
                        if self.current_room == self.map.ending_room {
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
            Command::Take { item } => {
                if self.item_in_current_room(item) {
                    self.take_item_from_current_room(item);
                    AdventureState::Success
                } else {
                    AdventureState::Failure {
                        error_msg: "That item is not in the room.".to_string(),
                    }
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
    use std::{fs, vec};

    use super::*;

    #[test]
    fn get_adventure_map_creates_map_correctly() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        let adventure_map = map.get_adventure_map().unwrap();

        assert_eq!(adventure_map.starting_room, "DormRoom");
        assert_eq!(adventure_map.ending_room, "LaundryRoom");

        // Ensure some rooms have correct names, descriptions, items, and directions
        let room_map = adventure_map.room_name_map;
        assert_eq!(room_map.len(), 7);

        let dorm_room = room_map.get(&adventure_map.starting_room).unwrap();
        assert_eq!(dorm_room.name, adventure_map.starting_room);

        let expected_description =
            "You are in your dorm room in Snyder Hall. Your stuff is littered everywhere."
                .to_string();
        assert_eq!(dorm_room.description, expected_description);

        let dorm_room_items = &dorm_room.items;
        let expected_items = vec!["bag", "computer", "clothes"];
        assert_eq!(dorm_room_items, &expected_items);

        let dorm_room_directions = &dorm_room.directions_map;
        assert_eq!(dorm_room_directions.len(), 1);

        let direction = dorm_room_directions.get("North").unwrap();
        assert_eq!(direction.direction_name, "North");
        assert_eq!(direction.destination, "SnyderHallway");
    }

    #[test]
    fn engine_loads_correctly() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map1: MapData = serde_json::from_str(&file).unwrap();
        let map2: MapData = serde_json::from_str(&file).unwrap();
        let starting_room = map1.starting_room.clone();
        let engine = AdventureEngine::new(map1).unwrap();

        assert_eq!(engine.map, map2.get_adventure_map().unwrap());
        assert_eq!(engine.current_room, starting_room);
    }

    #[test]
    fn adventure_room_take_item_works_on_valid_item() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        let mut engine = AdventureEngine::new(map).unwrap();

        let current_room: &mut AdventureRoom = engine.get_current_room_mut();

        // Assert initial state
        assert_eq!(
            current_room.items,
            vec![
                "bag".to_string(),
                "computer".to_string(),
                "clothes".to_string()
            ]
        );

        // Act
        let item = "computer".to_string();
        let result = current_room.take_item(&item);

        // Assert state post action
        assert_eq!(
            current_room.items,
            vec!["bag".to_string(), "clothes".to_string()]
        );
        assert_eq!(Ok(item), result);
    }

    #[test]
    fn adventure_room_drop_item_drops_correctly() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        let mut engine = AdventureEngine::new(map).unwrap();

        let current_room: &mut AdventureRoom = engine.get_current_room_mut();

        // Assert initial state
        assert_eq!(
            current_room.items,
            vec![
                "bag".to_string(),
                "computer".to_string(),
                "clothes".to_string()
            ]
        );

        // Act
        let item = "napkin".to_string();
        current_room.drop_item(item);

        // Assert state post action
        assert_eq!(
            current_room.items,
            vec![
                "bag".to_string(),
                "clothes".to_string(),
                "computer".to_string(),
                "napkin".to_string()
            ]
        );
    }

    #[test]
    fn adventure_room_item_exists_works_correctly() {
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        let mut engine = AdventureEngine::new(map).unwrap();

        let current_room: &mut AdventureRoom = engine.get_current_room_mut();

        // Assert initial state
        let expected_items = vec![
            "bag".to_string(),
            "computer".to_string(),
            "clothes".to_string(),
        ];
        assert_eq!(current_room.items, expected_items);

        // Act
        expected_items
            .iter()
            .for_each(|i| assert!(current_room.item_exists(i)));

        // Assert state post action
        assert_eq!(current_room.items, expected_items);
    }

    #[test]
    fn take_item_from_current_room_takes_item_correctly() {
        // Setup engine
        let file = fs::read_to_string("resources/dorm.json").unwrap();
        let map: MapData = serde_json::from_str(&file).unwrap();
        let mut engine = AdventureEngine::new(map).unwrap();

        // Assert initial state
        assert_eq!(
            engine.get_current_room().items,
            vec![
                "bag".to_string(),
                "computer".to_string(),
                "clothes".to_string()
            ]
        );
        let empty_inventory: Vec<String> = vec![];
        assert_eq!(engine.player.inventory, empty_inventory);

        // Act
        let item = "computer".to_string();
        engine.take_item_from_current_room(&item);

        // Assert state post action
        assert_eq!(
            engine.get_current_room().items,
            vec!["bag".to_string(), "clothes".to_string()]
        );
        assert_eq!(engine.player.inventory, vec!["computer".to_string()]);
    }
}
