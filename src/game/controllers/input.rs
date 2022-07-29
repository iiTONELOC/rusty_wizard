use crate::utils::{clear, quit};

pub fn input_controller(user_input: String) {
    let is_item = user_input.starts_with("get");
    let is_help = user_input.starts_with("help");
    let is_movement = user_input.starts_with("go");
    let is_quit = user_input.starts_with("q") || user_input.starts_with("exit");
    let is_clear = user_input.starts_with("clear") || user_input.starts_with("cls");

    if is_movement {
        println!("You selected movement");
    } else if is_item {
        return println!("You selected item");
    } else if is_quit {
        // why isn't this breaking?
        quit();
    } else if is_help {
        println!("You selected help");
    } else if is_clear {
        clear();
    } else {
        println!("Impossible cannot, {}, from here!", user_input);
    }
}
