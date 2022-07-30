use crate::game::game_data::constants::ITEM_NOT_FOUND;
use crate::game::game_data::models::Game;
use crate::game::utils::display::print;

fn get_args(user_input: &str) -> &str {
    return user_input.split("get").collect::<Vec<&str>>()[1].trim();
}

pub fn handle_add_to_inventory(user_input: &str, game: &mut Game) {
    let item = get_args(user_input).to_string();
    let available_items = &mut game._get_current_room().items.clone();
    let mut found = false;

    for current_item in available_items.iter() {
        if current_item == &item {
            found = true;
        }
    }

    if !found {
        print(ITEM_NOT_FOUND);
    } else {
        game._add_item_to_inventory(item.to_string());

        // filter out the item from the current room's items
        game.castle_rooms[game.current_room as usize].items = game
            ._get_current_room()
            .items
            .clone()
            .into_iter()
            .filter(|x| x != &item)
            .collect::<Vec<String>>();
    }
}
