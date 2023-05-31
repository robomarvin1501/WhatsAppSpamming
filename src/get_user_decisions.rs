use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct ScreenPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct UserSpecifications {
    pub file_or_stdin: String,
    pub message: String,
    pub time_period: f64,
    pub repeats: i32,
    pub browser_position: ScreenPosition,
    pub whatsapp_position: ScreenPosition,
}

pub fn get_user_specs() -> UserSpecifications {
    let file_or_stdin = get_file_or_stdin();
    let message = get_user_message(&file_or_stdin);
    let time_period = get_repeating_period();
    let repeats = get_number_of_repeats();
    let browser_position = get_x_and_y_position("the browser");
    let whatsapp_position = get_x_and_y_position("the whatsapp input bar");

    println!("Message: {message}");
    println!("Time period: {time_period}");
    println!("Repeats: {repeats}");

    let user_specs = UserSpecifications {
        file_or_stdin: file_or_stdin,
        message: message,
        time_period: time_period,
        repeats: repeats,
        browser_position: browser_position,
        whatsapp_position: whatsapp_position,
    };
    return user_specs;
}

fn get_single_position(prompt: &str) -> i32 {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut position: i32;

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    match stdin.read_line(&mut user_input) {
        Ok(_) => {
            user_input = String::from(user_input.trim());
        }
        Err(e) => {
            println!("ERROR: {e}");
            println!("Please try again");
        }
    };

    loop {
        let coordinate = user_input.parse::<i32>();
        match coordinate {
            Ok(_) => {
                position = coordinate.unwrap();
                break;
            }
            Err(why) => {
                println!("Input is not a integer: {}", why);
                user_input.clear();
                print!("{}", prompt);
                io::stdout().flush().unwrap();
            }
        };

        loop {
            match stdin.read_line(&mut user_input) {
                Ok(_) => {
                    user_input = String::from(user_input.trim());
                    break;
                }
                Err(e) => {
                    println!("ERROR: {e}");
                    println!("Please try again");
                }
            };
        }
    }

    return position;
}

fn get_x_and_y_position(target: &str) -> ScreenPosition {
    let x_position =
        get_single_position(format!("Please enter the x position of {}: ", target).as_str());
    let y_position =
        get_single_position(format!("Please enter the y position of {}: ", target).as_str());

    ScreenPosition {
        x: x_position,
        y: y_position,
    }
}

fn get_number_of_repeats() -> i32 {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let prompt = "How many times should this message be sent: ";

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    match stdin.read_line(&mut user_input) {
        Ok(_) => {
            user_input = String::from(user_input.trim());
        }
        Err(e) => {
            println!("ERROR: {e}");
            println!("Please try again");
        }
    };

    loop {
        let times_sent = user_input.parse::<i32>();
        match times_sent {
            Ok(_) => break,
            Err(why) => {
                println!("Input is not a integer: {}", why);
                user_input.clear();
                print!("{}", prompt);
                io::stdout().flush().unwrap();
            }
        };

        loop {
            match stdin.read_line(&mut user_input) {
                Ok(_) => {
                    user_input = String::from(user_input.trim());
                    break;
                }
                Err(e) => {
                    println!("ERROR: {e}");
                    println!("Please try again");
                }
            };
        }
    }
    println!("Times sent: {user_input}");
    user_input.parse::<i32>().unwrap()
}

fn get_user_message(target: &String) -> String {
    let mut user_acceptance = String::new();
    let stdin = io::stdin();
    let mut message: String;
    loop {
        if target == "f" {
            message = get_file_message();
        } else if target == "i" {
            message = get_stdin_message();
        } else {
            println!("INVALID target: {target}\n DEFAULTING TO STDIN");
            message = get_stdin_message();
        }
        print!("{message}\n\nIs this correct (y/n): ");
        io::stdout().flush().unwrap();

        loop {
            match stdin.read_line(&mut user_acceptance) {
                Ok(_) => break,
                Err(e) => {
                    println!("ERROR: {e}");
                    println!("Please try again");
                }
            };
        }
        user_acceptance = String::from(user_acceptance.trim());
        match user_acceptance.as_str() {
            "y" => break,
            _ => println!("+++ Not accepted by user +++"),
        };
        user_acceptance.clear();
    }

    return String::from(message.trim());
}

fn get_file_message() -> String {
    println!("NOT SUPPORTED: DEFAULTING TO GETTING FROM STDIN");
    let user_input = get_stdin_message();
    return user_input;
}

fn get_stdin_message() -> String {
    let mut user_input: String = String::new();
    let stdin = io::stdin();

    print!("Please write the message you wish to send: ");
    io::stdout().flush().unwrap();

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) => break,
            Err(e) => {
                println!("ERROR: {e}");
                println!("Please try again");
            }
        };
    }
    return user_input;
}

fn get_file_or_stdin() -> String {
    let mut user_input: String = String::new();
    let stdin = io::stdin();
    const POTENTIAL_READ_OPTIONS: [&str; 2] = ["f", "i"];

    print!("\nFile or user input (f/i): ");
    io::stdout().flush().unwrap();
    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) => break,
            Err(e) => panic!(" +++ ERROR +++ \n{}", e),
        };
    }

    user_input = String::from(user_input.trim());

    while !POTENTIAL_READ_OPTIONS.contains(&user_input.as_str()) {
        print!("{user_input} is not valid. Please try again, file or user input (f/i): ");
        io::stdout().flush().unwrap();
        user_input.clear();

        loop {
            match stdin.read_line(&mut user_input) {
                Ok(_) => break,
                Err(e) => panic!("+++ ERROR +++ \n{}", e),
            };
        }

        user_input = String::from(user_input.trim());
    }
    return user_input;
}

fn get_repeating_period() -> f64 {
    let mut user_input: String = String::new();
    let stdin = io::stdin();

    print!("How frequently should this message be sent (seconds): ");
    io::stdout().flush().unwrap();
    match stdin.read_line(&mut user_input) {
        Ok(_) => {
            user_input = String::from(user_input.trim());
        }
        Err(e) => {
            println!("ERROR: {e}");
            println!("Please try again");
        }
    };

    loop {
        // match user_input.chars().all(char::is_numeric) {
        let time_diff = user_input.parse::<f64>();
        match time_diff {
            Ok(_) => break,
            Err(why) => {
                println!("Input is not a number: {}", why);
                user_input.clear();
                print!("How frequently should this message be sent (seconds): ");
                io::stdout().flush().unwrap();
            }
        };

        loop {
            match stdin.read_line(&mut user_input) {
                Ok(_) => {
                    user_input = String::from(user_input.trim());
                    break;
                }
                Err(e) => {
                    println!("ERROR: {e}");
                    println!("Please try again");
                }
            };
        }
    }

    println!("Repeating time: {user_input}");
    user_input.parse::<f64>().unwrap()
}
