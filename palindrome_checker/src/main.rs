use std::io;
use std::process::exit;

fn main() {
    let entered_string: String;
    let reversed_string: String;

    println!("Welcome to the Palindrome Checker.");
    println!("Please enter a string not longer than 50 symbols:");
    entered_string = get_user_input();

    println!("You entered {}", entered_string);

    reversed_string = entered_string.trim().chars().rev().collect();

    if entered_string.trim() == reversed_string {
        println!("Yes, the string you entered, is a palindrome.")
    } else {
        println!("No, the string you entered is not a palidrome")
    }
}

/// Read user input into String
/// Verify length
/// Return String
fn get_user_input() -> String {
    let mut user_input: String = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(user_input) => user_input,
        Err(error) => panic!("Something went wrong, try again. {}", error),
    };

    if check_input_length(&user_input) {
        println!(
            "You entered {} characters, that is too long, try something shorter.",
            user_input.len()
        );

        exit(0);
    }

    user_input
}

/// Check String for length of less than
/// or equal to 50
fn check_input_length(input_string: &String) -> bool {
    if input_string.trim().len() >= 50 {
        true
    } else {
        false
    }
}
