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
        input::prompt_user,
    },
};

const ENEMY_ROOM: i8 = 3;

impl Game {
    pub fn new() -> Game {
        Game {
            user_inventory: Vec::new(),
            castle_rooms: get_rooms(),
            current_room: -1,
            score: 0,
            num_turns: -1,
        }
    }

    /// Starts a new game
    pub fn start(&mut self) {
        self._clear_terminal();
        self._reset();
        self._greet();

        self._play();
    }

    /// Runs the game on a loop
    fn _play(&mut self) {
        self._check_if_starting_round();
        self._loop()
    }

    /// Game loop
    fn _loop(&mut self) {
        let num_items = self._get_inventory().len();

        while self.current_room != ENEMY_ROOM && num_items < 6 {
            self._print_status();
            self._controller(self._prompt());
            self._increment_turns();
        }
    }

    /// Handles the user input and implements the correct behavior
    fn _controller(&mut self, user_input: String) {
        let _game = self;
        game_controller(user_input, _game);
    }

    // GETTERS and SETTERS

    /// get the current inventory
    fn _get_inventory(&self) -> &Vec<String> {
        &self.user_inventory
    }

    /// adds an item to inventory
    fn _add_item_to_inventory(&mut self, item: String) {
        self.user_inventory.push(item);
    }

    /// removes an item from inventory
    fn _remove_item_from_inventory(&mut self, index: usize) {
        self.user_inventory.remove(index);
    }

    /// get data about the current room
    fn _get_current_room(&self) -> &Room {
        &self.castle_rooms[self.current_room as usize]
    }

    /// set's the current room's index
    fn _set_current_room(&mut self, room: i8) {
        self.current_room = room;
    }

    /// prints the currents status of the game
    fn _print_status(&self) {
        print_status(
            self._get_current_room(),
            self._get_inventory(),
            self._get_num_turns() as usize,
        );
    }

    /// gets the current number of turns
    fn _get_num_turns(&self) -> i32 {
        self.num_turns
    }

    /// gets the score
    fn _get_score(&self) -> i32 {
        self._calculate_score()
    }

    // increment number of turns
    fn _increment_turns(&mut self) {
        self.num_turns += 1;
    }

    // calculate score
    fn _calculate_score(&self) -> i32 {
        let score = (100000000 - self.num_turns) * 10;
        score
    }

    // UTILITY FUNCTIONS

    fn _reset(&mut self) {
        // reset game state
        self.user_inventory = Vec::new();
        self.castle_rooms = get_rooms();
        self.current_room = -1;
        self.num_turns = -1;
        self.score = 0;
    }

    fn _prompt(&self) -> String {
        let msg = prompt_user("\nWhat would you like to do?\n");
        print_horizontal_rule();
        msg
    }

    fn _check_if_starting_round(&mut self) {
        if self.current_room == -1 {
            self.current_room += 1;
            self.num_turns += 2;
        }
    }

    fn _greet(&self) {
        greet_user();
    }

    fn _clear_terminal(&self) {
        clear()
    }
}
