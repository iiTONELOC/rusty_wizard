mod controller;
pub mod game_data;
mod utils;

use self::{
    controller::game_controller,
    game_data::{
        get_rooms,
        models::{Game, Room},
    },
    utils::{
        display::{clear, greet_user, print_horizontal_rule, print_status},
        ending::handle_ending,
        input::prompt_user,
        score::calculate_score,
    },
};

const ENEMY_ROOM: i8 = 3;

impl Game {
    pub fn new() -> Game {
        Game {
            user_inventory: Vec::new(),
            castle_rooms: get_rooms(),
            current_room: -1,
            num_turns: -1,
            score: 0,
        }
    }

    pub fn start(&mut self) {
        self._reset();
        self._greet();
        self._play();
    }

    fn _play(&mut self) {
        self._check_if_starting_round();
        self._loop()
    }

    // _______ INTERNAL API _________

    fn _loop(&mut self) {
        while self.current_room != ENEMY_ROOM && self._get_inventory().len() < 6 {
            self._print_status();
            self._controller(self._prompt());
            self._increment_turns();
        }

        handle_ending(self);
    }

    fn _controller(&mut self, user_input: String) {
        game_controller(user_input, self);
    }

    // _______ GETTERS & SETTERS _________
    //____________________________________

    fn _get_inventory(&self) -> Vec<String> {
        self.user_inventory.clone()
    }

    fn _get_num_turns(&self) -> i32 {
        self.num_turns
    }

    fn _get_score(&mut self) -> i32 {
        self.score
    }

    fn _get_current_room(&self) -> &Room {
        &self.castle_rooms[self.current_room as usize]
    }

    fn _set_score(&mut self, score: i32) {
        self.score = score;
    }

    fn _set_item_in_inventory(&mut self, item: String) {
        self.user_inventory.push(item);
    }

    fn _set_current_room(&mut self, room: i8) {
        self.current_room = room;
    }

    // _______ END GETTERS & SETTERS _________
    //________________________________________

    // _______ UTILITY FUNCTIONS _________
    //____________________________________

    fn _reset(&mut self) {
        // reset game state
        self.user_inventory = Vec::new();
        self.castle_rooms = get_rooms();
        self.current_room = -1;
        self.num_turns = -1;
        self.score = 0;
    }

    fn _print_status(&self) {
        print_status(
            self._get_current_room(),
            &self._get_inventory(),
            self._get_num_turns() as usize,
        );
    }

    fn _prompt(&self) -> String {
        let msg = prompt_user("\nWhat would you like to do?\n");
        print_horizontal_rule();
        msg
    }

    fn _check_if_starting_round(&mut self) {
        if self.current_room == -1 {
            self._set_current_room(0);
            self.num_turns += 2;
        }
    }

    fn _increment_turns(&mut self) {
        self.num_turns += 1;
    }

    fn _calculate_score(&mut self) -> i32 {
        calculate_score(self)
    }

    fn _greet(&self) {
        greet_user();
    }

    fn _clear_terminal(&self) {
        clear();
    }
}
