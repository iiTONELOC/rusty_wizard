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
