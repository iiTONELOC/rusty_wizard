use super::quit;

pub fn input_controller(user_input: &str) {
    let is_movement = user_input.starts_with("go");
    let is_item = user_input.starts_with("get");
    let is_quit = user_input.starts_with("q") || user_input.starts_with("exit");
    let is_help = user_input.starts_with("help");

    if is_movement {
        println!("You selected movement");
    } else if is_item {
        return println!("You selected item");
    } else if is_quit {
        // why isn't this breaking?
        quit();
    } else if is_help {
        println!("You selected help");
    } else {
        println!("Invalid input");
    }
}
