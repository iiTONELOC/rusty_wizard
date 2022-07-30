#[derive(Debug, Clone)]
pub struct Movement {
    pub direction: &'static str,
    pub move_to: i8,
}

#[derive(Debug)]
pub struct Room {
    pub name: &'static str,
    pub description: &'static str,
    pub directional_moves: Vec<Movement>,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub struct RoomData {
    pub rooms: Vec<Room>,
}

#[derive(Debug)]
pub struct Game {
    pub user_inventory: Vec<String>,
    pub current_room: i8,
    pub castle_rooms: Vec<Room>,
    pub score: i32,
    pub num_turns: i32,
}
