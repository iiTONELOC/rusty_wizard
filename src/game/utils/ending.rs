use crate::game::{
    game_data::models::Game,
    utils::{input::prompt_user, quit},
};

pub fn handle_ending(game: &mut Game) {
    let num_items = game._get_inventory().len();
    let item_string = if num_items == 1 { "item" } else { "items" };

    game._calculate_score();

    if num_items == 6 {
        println!("\nCongratulations, you have defeated the evil Wizard!");
    } else {
        println!(
            "\nYou found the Wizard but you only have {} {}. Collect all 6 items to win!\n",
            num_items, item_string
        );
        println!("Game Over!\n");
    }

    println!("You took {} turns", game._get_num_turns());
    println!("Your score is: {}\n", game._get_score());
    println!("Thanks for playing!\n");

    let want_to_play_again = prompt_user("Would you like to play again? (y/n) ");

    if want_to_play_again.to_lowercase().starts_with("y") {
        game.start();
    } else {
        quit();
    }
}
