mod game_data;
use game_data::get_rooms;

fn main() {
    let rooms = get_rooms();

    for room in rooms {
        println!("{}", room.description);
    }
}
