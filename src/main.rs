use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use enigo::*;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        // Ok(_) => print!("{} contains:\n{}", display, s),
        Ok(_) => print_by_character(&s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn print_by_character(message: &str) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta);
    enigo.key_down(Key::Layout('1'));
    enigo.key_up(Key::Layout('1'));
    enigo.key_up(Key::Meta);
    for letter in message.chars() {
        if letter != ' ' && letter != '\n' {
            enigo.key_click(Key::Layout(letter));
            enigo.key_click(Key::Return);
        }
    }
}
