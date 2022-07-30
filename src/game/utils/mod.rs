pub mod display;
pub mod ending;
pub mod input;
pub mod inventory;
pub mod movement;
pub mod score;

use display::print;

/// exit the game
pub fn quit() {
    print("\nGoodbye!\n");
    std::process::exit(0);
}
