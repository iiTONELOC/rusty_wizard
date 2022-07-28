use crate::game_data::models::Directions;

// we can store the numbers we need once and then use them in multiple places
pub const NUMBERS: [i8; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

// Constants for Movement
const NORTH: &'static str = "North";
const SOUTH: &'static str = "South";
const EAST: &'static str = "East";
const WEST: &'static str = "West";

pub const DIRECTIONS: Directions = Directions {
    north: &NORTH,
    south: &SOUTH,
    east: &EAST,
    west: &WEST,
};

pub const ROOM_NAMES: [&'static str; 13] = [
    "Meeting Room",
    "Gallery",
    "Conservatory",
    "Sitting Room",
    "Trophy Hall",
    "Study",
    "Summoning Room",
    "Court Yard",
    "Workshop",
    "Foundry",
    "Master's Chamber",
    "Bath",
    "Kitchen",
];

const NULL: &'static str = "null";

pub const ROOM_ITEMS: [&'static str; 13] = [
    &NULL,
    &NULL,
    "Maps of the Echo Lands",
    &NULL,
    &NULL,
    "Locket of Reflection",
    "Ring of Dispel",
    &NULL,
    "Mermaid Tears",
    "Tablet of Destiny",
    &NULL,
    "Flaming Pearl",
    &NULL,
];
