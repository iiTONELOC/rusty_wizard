pub mod game_data;
mod utils;

use self::game_data::get_rooms;
use self::game_data::models::{Game, Room};
use self::utils::display::greet_user;
use self::utils::movement::handle_movement;
use self::utils::{clear, prompt_user, quit};

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
            self._print_description();
            self._controller(self._prompt());
            self._increment_turns();
        }
    }

    /// Handles the user input and implements the correct behavior
    fn _controller(&mut self, user_input: String) {
        let is_item = user_input.starts_with("get");
        let is_help = user_input.starts_with("help");
        let is_movement = user_input.starts_with("go");
        let is_quit = user_input.starts_with("q") || user_input.starts_with("exit");
        let is_clear = user_input.starts_with("clear") || user_input.starts_with("cls");

        if is_movement {
            let next_room_number =
                handle_movement(&user_input, self._get_current_room(), self.current_room);

            self._set_current_room(next_room_number);
        } else if is_item {
            println!("You selected item");
        } else if is_quit {
            quit();
        } else if is_help {
            println!("You selected help");
        } else if is_clear {
            clear();
            return self._play();
        } else {
        }
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

    /// prints the description of the room
    fn _print_description(&self) {
        let items_in_room = &self._get_current_room().items;

        let room_description = if items_in_room.len() > 0 {
            let mut description = self._get_current_room().description.to_string();
            description.push_str("\nYou see an item, the ");

            description
        } else {
            self._get_current_room().description.to_string()
        };

        if items_in_room.len() > 0 {
            println!("{}{}\n", room_description, items_in_room.join(", "));
        } else {
            println!("{}", room_description);
        }
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
        prompt_user("What would you like to do? ")
    }

    fn _check_if_starting_round(&mut self) {
        if self.current_room == -1 {
            self.current_room += 1;
            self._increment_turns();
        }
    }

    fn _greet(&self) {
        greet_user();
    }

    fn _clear_terminal(&self) {
        clear()
    }
}
