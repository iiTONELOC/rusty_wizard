use crate::game::{
    game_data::models::Game,
    utils::{
        display::{clear, print_help},
        inventory::handle_add_to_inventory,
        movement::handle_movement,
        quit,
    },
};

pub fn game_controller(user_input: String, game: &mut Game) {
    let is_item = user_input.starts_with("get");
    let is_help = user_input.starts_with("help");
    let is_movement = user_input.starts_with("go");
    let is_quit = user_input.starts_with("q") || user_input.starts_with("exit");
    let is_clear = user_input.starts_with("clear") || user_input.starts_with("cls");

    if is_movement {
        game._set_current_room(handle_movement(
            &user_input,
            game._get_current_room(),
            game.current_room,
        ));
    } else if is_item {
        handle_add_to_inventory(&user_input, game);
    } else if is_quit {
        quit();
    } else if is_help {
        print_help();
        // The round increments after user input
        // A user shouldn't be penalized for asking for help
        // Rather than write more code to handle this, we'll just decrement
        // num_turns by 1 to correct this.
        game.num_turns -= 1;
    } else if is_clear {
        clear();
        return game._play();
    } else {
    }
}
