use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// A struct that holds an x and y coordinate that should be within the screen bounds
#[derive(Debug)]
pub struct ScreenPosition {
    pub x: i32,
    pub y: i32,
}

/// A struct that holds the choices made by the user with regards to how the spammer should be run
#[derive(Debug)]
pub struct UserSpecifications {
    pub file_or_stdin: String,
    pub message: String,
    pub time_period: f64,
    pub repeats: i32,
    pub browser_position: ScreenPosition,
    pub whatsapp_position: ScreenPosition,
}

/// Wrapper function exposed to main.rs that gets all of the information from the user, and then
/// returns it all in [`UserSpecifications`]
///
/// # Examples
/// ```
/// let user_specs: UserSpecifications = get_user_specs();
/// ```
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

/// Gets a single integer position on the screen
/// TODO does not check that the position is within the bounds of the screen.
///
/// # Examples
/// ```
/// let x_position = get_single_position("Please input the x position of the browser: ");
/// let y_position = get_single_position("Please input the y position of the browser: ");
/// ```
fn get_single_position(prompt: &str) -> i32 {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let position: i32;

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

/// Returns the ScreenPosition struct storing the position of a given target
/// Also takes the name of the target, with any relevant prefixes to be passed to the prompt called
/// in [`get_single_position()`]
///
/// # Examples
/// ```
/// let browser_position: ScreenPosition = get_x_and_y_position("the browser");
/// let whatsapp_position: ScreenPosition = get_x_and_y_position("whatsapp");
/// ```
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

/// Get integer user input. This is aimed at getting how many times the message should be repeated
///
/// # Examples
/// ```
/// let repeats: i32 = get_number_of_repeats();
/// ```
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

/// Get the message from the user from either a file or stdin.
/// Files are currently unsupported and default to stdin.
///
/// # Examples
/// ```
/// let specs: UserSpecifications = get_user_specs();
/// let message: String = get_user_message(&specs.file_or_stdin);
/// ```
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

/// Get the message from a file. 
/// Automatically requests a path from the user.
///
/// # Examples
/// ```
/// let message: String - get_file_message();
/// ```
fn get_file_message() -> String {
    let mut user_input: String = String::new();
    let mut file_path: &Path;
    let stdin = io::stdin();

    print!("Please write the path to the file you wish to send: ");
    io::stdout().flush().unwrap();

    loop {
        match stdin.read_line(&mut user_input) {
            Ok(_) => {
                user_input = String::from(user_input.trim());
                break;
            },
            Err(e) => {
                println!("ERROR: {e}");
                println!("Please try again");
                print!("Please write the path to the file you wish to send: ");
                io::stdout().flush().unwrap();
            }
        }
    }

    loop {
        file_path = Path::new(&user_input);
        if file_path.exists() {
            break;
        } else {
            println!("ERROR: Path {user_input} does not exist!");

            loop {
                match stdin.read_line(&mut user_input) {
                    Ok(_) => break,
                    Err(e) => {
                        println!("ERROR: {e}");
                        println!("Please try again");
                        print!("Please write the path to the file you wish to send: ");
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }
    }
    match read_message_from_file(file_path) {
        Ok(result) => result,
        Err(_) => panic!("Error reading provided file"),
    }
}

/// Gets a message from the user via stdin, without failing.
///
/// # Examples
/// ```
/// let message: String = get_stdin_message();
/// ```
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

/// Gets whether or not the user wishes to use stdin or a file to create the message that they are
/// sending.
/// Will only return the available options, repeatedly doesn't accept anything else.
///
/// # Examples
/// ```
/// let file_or_stdin: String = get_file_or_stdin();
/// ```
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

/// Gets the period between messages
///
/// ```
/// let repeating_period: f64 = get_repeating_period();
/// ```
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
/// This function reads a message from the given file, and returns it
///
/// # Examples
/// ```
/// let message: String = read_message_from_file(Path::new("/path/to/file"));
/// ```
fn read_message_from_file(path: &Path) -> Result<String, io::Error> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut message = String::new();
    match file.read_to_string(&mut message) {
        Ok(_) => Ok(message),
        Err(e) => Err(e),
    }
}
