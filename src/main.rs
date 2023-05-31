use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{env, io};

use enigo::*;

mod get_user_decisions;
mod show_screen_coords;

// TODO needs a failsafe of some sort

fn main_old() {
    // Create a path to the desired file
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        println!("INSUFFICIENT ARGUMENTS");
        println!("Expected 5 arguments, received {}", args.len());
        println!("Arguments should be <message file path>, <browser_x>, <browser_y>, whatsapp message bar x>, <whatsapp message bar y>");
        panic!("INSUFFICIENT ARGUMENTS");
    }
    println!("Reading from file: {}", &args[1]);
    let path = Path::new(&args[1]);
    // let display = path.display();

    let message = read_message_from_file(path);
    println!("{}", message);

    print_whole_message(
        &message,
        args[2].parse::<i32>().unwrap(),
        args[3].parse::<i32>().unwrap(),
        args[4].parse::<i32>().unwrap(),
        args[5].parse::<i32>().unwrap(),
    )
}

#[derive(Debug)]
enum WhichMain {
    GetCoords,
    Spam,
    Exit,
}

fn main() {
    let mut user_choice = coords_run_or_exit();
    println!("{:?}", user_choice);

    loop {
        match user_choice {
            WhichMain::Exit => return,
            WhichMain::GetCoords => show_screen_coords::show_screen_coords(),
            WhichMain::Spam => {
                let specs = get_user_decisions::get_user_specs();
                println!("{:?}", specs);
                print_whole_message(
                    specs.message.as_str(),
                    specs.browser_position.x,
                    specs.browser_position.y,
                    specs.whatsapp_position.x,
                    specs.whatsapp_position.y,
                )
            }
        }
        user_choice = coords_run_or_exit();
    }
}

fn coords_run_or_exit() -> WhichMain {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut choice: WhichMain;

    println!("Please select a run option: ");
    println!("Show screen coordinates: (c)\nStart the spammer: (s)\nExit: (e)");

    loop {
        match user_input.as_str() {
            "c" => {
                choice = WhichMain::GetCoords;
                break;
            }
            "s" => {
                choice = WhichMain::Spam;
                break;
            }
            "e" => {
                choice = WhichMain::Exit;
                break;
            }
            "" => {}
            _ => println!("{}: Not a valid choice", user_input),
        };

        loop {
            user_input.clear();
            print!("Your selection (c/s/e): ");
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

fn print_by_character(message: &str) {
    let clean_message: String = message.chars().filter(|c| !c.is_whitespace()).collect();

    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout('1'));
    enigo.key_up(Key::Meta);

    for letter in clean_message.chars() {
        enigo.key_click(Key::Layout(letter));
        enigo.key_click(Key::Return);
    }
}

fn print_whole_message(
    message: &str,
    browser_x: i32,
    browser_y: i32,
    whatsapp_x: i32,
    whatsapp_y: i32,
) {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(browser_x, browser_y);
    enigo.mouse_click(MouseButton::Left);

    enigo.mouse_move_to(whatsapp_x, whatsapp_y);
    enigo.mouse_click(MouseButton::Left);

    enigo.key_sequence(message);
    enigo.key_click(Key::Return);
}
