pub mod constants;
pub mod models;

use self::constants::{available_moves, ROOM_DESCRIPTIONS, ROOM_NAMES};
use self::models::{Room, RoomData};

pub fn get_rooms() -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();

    for i in 0..ROOM_NAMES.len() {
        let _description =
            "\nYou are in the ".to_string() + ROOM_NAMES[i] + ".\n" + ROOM_DESCRIPTIONS[i];

        rooms.push(Room {
            name: ROOM_NAMES[i].to_string(),
            description: _description,
            directional_moves: available_moves()[i].clone(),
            items: Vec::new(),
        });
    }

    let room_data = RoomData { rooms };

    return room_data.rooms;
}
