pub mod display;
use regex::Regex;

// https://rust-lang.github.io/regex/regex/index.html#character-classes
const REGEX_PATTERN: &str = r"^[[:alpha:]\s]*$";
const INVALID_INPUT_MESSAGE: &str = "Invalid input. Please try again.";

// helper function to validate user input
// Examines the input string and returns true if it matches the regex pattern and isn't empty whitespace.
fn is_valid_input(input: &str, regex: &Regex) -> bool {
    return regex.is_match(input) && input.to_string().trim().len() > 0;
}

// validates user input and returns the input if it matches the regex pattern.
// if the input does not match the regex pattern, the user is prompted to try again.
fn validate_input(_input: String) -> String {
    let reg_ex: Regex = Regex::new(&REGEX_PATTERN).unwrap();
    let mut is_valid = is_valid_input(&_input, &reg_ex);
    let mut response = _input;

    while !is_valid {
        print(&INVALID_INPUT_MESSAGE);
        response = input();
        is_valid = is_valid_input(&response, &reg_ex);
    }

    return response;
}

// internal function to retrieve user input
fn input() -> String {
    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response).unwrap();
    return user_response.trim().to_string();
}

// internal function
// displays a message to the user
// and takes input from the use
fn input_with_message(message: &str) -> String {
    print(message);
    return input();
}

// reusable dynamic println! function
pub fn print(message: &str) {
    println!("{}", message);
}

// prompt the user for input and validate the input
// the input is validated to be a valid string of alphabetic characters and spaces only
// it is guaranteed to not be empty.
pub fn prompt_user(message: &str) -> String {
    let original_input = input_with_message(message);
    let user_input = validate_input(original_input);
    return user_input;
}

pub fn quit() {
    print("Goodbye!");
    std::process::exit(0);
}

pub fn clear() {
    print("\x1B[2J\x1B[1;1H");
}
