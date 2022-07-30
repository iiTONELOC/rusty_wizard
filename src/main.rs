mod game;
use self::game::game_data::models::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
