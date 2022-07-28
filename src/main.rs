mod game_data;
use game_data::get_rooms;

fn main() {
    let room_data = get_rooms();
    println!("Hello, world!");

    println!("{:?}", room_data);
}
