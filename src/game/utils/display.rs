use crate::game::game_data::models::Room;

const TITLE: &'static str = "
.______        ___      .__   __.  __       _______. __    __   _______ .______
|   _  \\      /   \\     |  \\ |  | |  |     /       ||  |  |  | |   ____||   _  \\
|  |_)  |    /  ^  \\    |   \\|  | |  |    |   (----`|  |__|  | |  |__   |  |_)  |
|   _  <    /  /_\\  \\   |  . `  | |  |     \\   \\    |   __   | |   __|  |      / 
|  |_)  |  /  _____  \\  |  |\\   | |  | .----)   |   |  |  |  | |  |____ |  |\\  \\--.
|______/  /__/     \\__\\ |__| \\__| |__| |_______/    |__|  |__| |_______|| _| `.___|
";

const TITLE_MESSAGE: &'static str =
    "Welcome Merlin! The evil wizard, Atlantes has ransacked the castle and is hiding \nwithin!\n
You must collect all 6 items before facing Atlantes Or you will be exiled from \nthis realm!";

const HELP_MESSAGE: &'static str = "\nYou may move in the four basic directions the commands are:
\"go North\", \"go South\", \"go East\", and \"go West\".\n
To retrieve an item enter: \"get <item name>\".\n
Each selection is a move and you have one move per turn. Choose wisely!
You can type \"help\" at anytime to see the commands again or \"quit\" to exit.
";

const COL_WIDTH: usize = 82;

/// reusable dynamic println! function
pub fn print(message: &str) {
    println!("{}", message);
}

pub fn print_help() {
    print(&HELP_MESSAGE);
    print(&"_".repeat(COL_WIDTH).as_str());
}

pub fn greet_user() {
    print(&TITLE);
    print(&TITLE_MESSAGE);

    print_help();
}

/// clears the terminal and resets the cursor position
pub fn clear() {
    print("\x1B[2J\x1B[1;1H");
    print(TITLE);
}

pub fn print_horizontal_rule() {
    print("_".repeat(COL_WIDTH).as_str());
}

pub fn print_status(room: &Room, player_inventory: &Vec<String>, num_turns: usize) {
    let items_in_room = &room.items;

    // Generate the description of the room
    let room_description = if items_in_room.len() > 0 {
        let mut description = room.description.to_string();
        description.push_str("\nYou see an item, the ");

        description
    } else {
        room.description.to_string()
    };

    // Print the description of the room
    if items_in_room.len() > 0 {
        println!("{}{}\n", room_description, items_in_room.join(", "));
    } else {
        println!("{}", room_description);
    }

    print(&format!("\nInventory: {:?}\n", player_inventory));
    print(&format!("Turns: {}", num_turns));
    print_horizontal_rule();
}
