pub mod available_moves;
pub mod constants;

pub mod models;

use self::{
    available_moves::available_moves,
    constants::{NULL, ROOM_DESCRIPTIONS, ROOM_ITEMS, ROOM_NAMES},
    models::{Room, RoomData},
};

pub fn get_rooms() -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();

    for i in 0..ROOM_NAMES.len() {
        // while this could be just a string,
        // its an array for expandability, we could always add more items and
        // have the user acquire certain items to defeat the enemy.

        let mut _room_items: Vec<String> = Vec::new();

        if ROOM_ITEMS[i] != NULL {
            _room_items.push(ROOM_ITEMS[i].to_string());
        }

        rooms.push(Room {
            name: &ROOM_NAMES[i],
            description: &ROOM_DESCRIPTIONS[i],
            directional_moves: available_moves()[i].clone(),
            items: _room_items,
        });
    }

    let room_data = RoomData { rooms };
    return room_data.rooms;
}
