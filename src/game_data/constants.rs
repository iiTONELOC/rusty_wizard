use crate::game_data::models::{Directions, Movement};

// Constants for Movement
const NORTH: &'static str = "North";
const SOUTH: &'static str = "South";
const EAST: &'static str = "East";
const WEST: &'static str = "West";

pub const DIRECTIONS: Directions = Directions {
    north: NORTH,
    south: SOUTH,
    east: EAST,
    west: WEST,
};

pub const ROOM_NAMES: [&'static str; 4] = ["Meeting Room", "Room 2", "Room 3", "Room 4"];

pub const ROOM_DESCRIPTIONS: [&'static str; 4] = [
    "
There is a large table in the center of the room.  
You see doors to the north and east and windows to the south and west
    ",
    "This is Room 2",
    "This is Room 3",
    "This is Room 4",
];

pub fn available_moves() -> [Vec<Movement>; 4] {
    [
        vec![Movement {
            direction: DIRECTIONS.north,
            move_to: 1,
        }],
        vec![Movement {
            direction: DIRECTIONS.south,
            move_to: 0,
        }],
        vec![Movement {
            direction: DIRECTIONS.east,
            move_to: 2,
        }],
        vec![Movement {
            direction: DIRECTIONS.west,
            move_to: 3,
        }],
    ]
}
