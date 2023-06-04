use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use enigo::*;
use get_user_decisions::UserSpecifications;

mod get_user_decisions;
mod show_screen_coords;

// TODO needs a failsafe of some sort

/// [`WhichMain`] represents the user's choice of what part of the program to run, represented by an enum  
/// Used by [`coords_run_or_exit()`]
#[derive(Debug)]
enum WhichMain {
    GetCoords,
    SpamWholeMessage,
    SpamWords,
    SpamLetters,
    Exit,
}

fn main() {
    let mut user_choice = coords_run_or_exit();
    println!("{:?}", user_choice);

    loop {
        match user_choice {
            WhichMain::Exit => return,
            WhichMain::GetCoords => show_screen_coords::show_screen_coords(),
            WhichMain::SpamWholeMessage => {
                let specs = get_user_decisions::get_user_specs();
                println!("{:?}", specs);
                for _ in 0..specs.repeats {
                    print_whole_message(&specs);
                    sleep(Duration::from_millis((specs.time_period * 1000.0) as u64));
                }
            }
            WhichMain::SpamLetters => {
                let specs = get_user_decisions::get_user_specs();
                println!("{:?}", specs);
                for _ in 0..specs.repeats {
                    print_by_character(&specs)
                }
            }
            WhichMain::SpamWords => {
                let specs = get_user_decisions::get_user_specs();
                println!("{:?}", specs);
                for _ in 0..specs.repeats {
                    print_by_word(&specs);
                    sleep(Duration::from_millis((specs.time_period * 1000.0) as u64));
                }
            }
        }
        user_choice = coords_run_or_exit();
    }
}

/// Gets from the user what part of the program to run, eg show the coordinates
/// spamming options, or just exiting
///
/// Returns [`WhichMain`], to hold the user choice. This may be matched
/// against for their input
///
///
/// # Examples
/// ```
/// let user_choice: WhichMain = coords_run_or_exit();
/// match user_choice {
///     WhichMain::SpamWords => println!("User selected to spam words!"),
///     _ => {},
/// }
/// ```
fn coords_run_or_exit() -> WhichMain {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let choice: WhichMain;

    println!("Please select a run option: ");
    println!(
        "{}\n{}\n{}\n{}\n{}",
        "Show screen coordinates: (c)",
        "Spam a whole message at once (m)",
        "Spam word by word (w)",
        "Spam letter by letter (l)",
        "Exit: (e)"
    );

    loop {
        match user_input.as_str() {
            "c" => {
                choice = WhichMain::GetCoords;
                break;
            }
            "m" => {
                choice = WhichMain::SpamWholeMessage;
                break;
            }
            "w" => {
                choice = WhichMain::SpamWords;
                break;
            }
            "l" => {
                choice = WhichMain::SpamLetters;
                break;
            }
            "e" => {
                choice = WhichMain::Exit;
                break;
            }
            "" => {}
            _ => println!("{}: Not a valid choice", user_input),
        };

        // Get the user input, and don't fail if they give an invalid one.
        // Just keep trying till you get a good one
        loop {
            user_input.clear();
            print!("Your selection (c/m/w/l/e): ");
            io::stdout().flush().unwrap();
            match stdin.read_line(&mut user_input) {
                Ok(_) => {
                    user_input = String::from(user_input.trim());
                    break;
                }
                Err(e) => {
                    println!("ERROR: {e}");
                    println!("Please try again");
                }
            }
        }
    }

    return choice;
}

/// This function reads a message from the given file, and returns it
/// This code will be used again at some point, but for the moment languishes, friendless.
///
/// # Examples
/// ```
/// let message: String = read_message_from_file(Path::new("/path/to/file"));
/// ```
fn read_message_from_file(path: &Path) -> String {
    // Open the path in read-only mode, returns `io::Result<File>`
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}

/// Sends a message word by word to the recipient
/// Each word is sent as fast as possible, with a potential gap between each message as dictated by
/// the user. This however is handled seperately.
///
/// # Examples
/// ```
/// let specs: UserSpecifications = UserSpecifications{"Never gonna give you up", 20, 1900, 989, 941};
/// print_by_word(&specs);
/// ```
fn print_by_word(user_specs: &UserSpecifications) {
    let words = user_specs.message.as_str().split_whitespace();

    let mut enigo = Enigo::new();
    //Move to browser and click.
    enigo.mouse_move_to(user_specs.browser_position.x, user_specs.browser_position.y);
    enigo.mouse_click(MouseButton::Left);

    //Move mouse to whatsapp position.
    enigo.mouse_move_to(
        user_specs.whatsapp_position.x,
        user_specs.whatsapp_position.y,
    );
    enigo.mouse_click(MouseButton::Left);

    //Print each word
    for word in words {
        enigo.key_sequence(word);
        enigo.key_click(Key::Return);
    }
}

/// Sends a message character by character to the recipient
/// Each character is sent as fast as possible, with a potential gap between each message as dictated by
/// the user. This however is handled seperately.
///
/// # Examples
/// ```
/// let specs: UserSpecifications = UserSpecifications{"Never gonna give you up", 20, 1900, 989, 941};
/// print_by_character(&specs);
/// ```
fn print_by_character(user_specs: &UserSpecifications) {
    let clean_message: String = user_specs
        .message
        .as_str()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut enigo = Enigo::new();
    enigo.mouse_move_to(user_specs.browser_position.x, user_specs.browser_position.y);
    enigo.mouse_click(MouseButton::Left);

    enigo.mouse_move_to(
        user_specs.whatsapp_position.x,
        user_specs.whatsapp_position.y,
    );
    enigo.mouse_click(MouseButton::Left);

    for letter in clean_message.chars() {
        enigo.key_click(Key::Layout(letter));
        enigo.key_click(Key::Return);
    }
}

/// Sends the whole message at a time, with the interval between messages handled by the user
/// elsewhere.
///
/// # Examples
/// ```
/// let specs: UserSpecifications = UserSpecifications{"Never gonna give you up", 20, 1900, 989, 941};
/// print_whole_message(&specs);
/// ```
fn print_whole_message(user_specs: &UserSpecifications) {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(user_specs.browser_position.x, user_specs.browser_position.y);
    enigo.mouse_click(MouseButton::Left);

    enigo.mouse_move_to(
        user_specs.whatsapp_position.x,
        user_specs.whatsapp_position.y,
    );
    enigo.mouse_click(MouseButton::Left);

    enigo.key_sequence(user_specs.message.as_str());
    enigo.key_click(Key::Return);
}
