mod game_data;
mod utils;

use game_data::get_rooms;
use utils::display::greet_user;
use utils::prompt_user;
use utils::user_input::input_controller;
fn main() {
    get_rooms();

    greet_user();
    input_controller(&prompt_user("What is your name?"));
}
