use crate::game::game_data::constants::{
    DIRECTION_NOT_AVAILABLE, EAST, INVALID_DIRECTION, NORTH, NULL, SOUTH, WEST,
};
use crate::game::game_data::models::Room;
use crate::game::utils::print;

/// Parses the user input and returns the direction the user wants to move to.
/// Returns "null" if the user input is invalid.
fn get_direction(user_input: &str) -> &str {
    let directions = [NORTH, SOUTH, EAST, WEST];

    for direction in directions.iter() {
        let tokens = user_input.split(" ").collect::<Vec<&str>>();
        if tokens[1].to_string().to_lowercase() == direction.to_string().to_lowercase() {
            return direction;
        }
    }

    return NULL;
}

/// returns the next room number based on the direction provided
/// if the direction is not valid, it returns the current room number
fn get_next_room_number(direction: &str, current_room: &Room, current_room_number: i8) -> i8 {
    // see if the desired location is listed in the room.directional_moves

    for room_exit in current_room.directional_moves.iter() {
        if room_exit.direction == direction {
            return room_exit.move_to;
        }
    }

    if direction == NULL {
        print(INVALID_DIRECTION);
    } else {
        let msg = &DIRECTION_NOT_AVAILABLE.replace("that way", &direction.to_lowercase());
        print(msg);
    }

    return current_room_number;
}

/// Returns the next room number based on the direction provided.
pub fn handle_movement(user_input: &str, current_room: &Room, current_room_number: i8) -> i8 {
    let direction = get_direction(user_input);
    return get_next_room_number(direction, current_room, current_room_number);
}
