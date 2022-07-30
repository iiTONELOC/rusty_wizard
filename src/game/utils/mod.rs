pub mod display;
pub mod input;
pub mod inventory;
pub mod movement;

use display::print;

/// exit the game
pub fn quit() {
    print("\nGoodbye!\n");
    std::process::exit(0);
}
