use crate::game::game_data::models::Game;

pub fn calculate_score(game: &mut Game) -> i32 {
    if game._get_inventory().len() == 6 {
        // check number of turns
        let num_turns = game._get_num_turns();

        let multiplier: i32 = if num_turns < 22 {
            1275
        } else if num_turns < 30 {
            950
        } else if num_turns < 40 {
            775
        } else if num_turns < 50 {
            650
        } else {
            225
        };

        game._set_score(multiplier * num_turns);
    } else {
        game._set_score(game._get_num_turns() * 5);
    }
    game._get_score()
}
