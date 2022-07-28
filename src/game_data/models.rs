#[derive(Debug)]
pub struct Directions {
    pub north: &'static str,
    pub south: &'static str,
    pub east: &'static str,
    pub west: &'static str,
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub direction: &'static str,
    pub move_to: i8,
}

#[derive(Debug)]
pub struct Room {
    pub name: &'static str,
    pub directional_moves: Vec<Movement>,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub struct RoomData {
    pub rooms: Vec<Room>,
}
