mod controllers;
mod game_data;
mod utils;

use controllers::user_input::input_controller;
use game_data::get_rooms;
use utils::display::greet_user;
use utils::prompt_user;

fn main() {
    get_rooms();

    greet_user();
    let user_input = prompt_user("Please enter your name: ");
    input_controller(user_input);
}
