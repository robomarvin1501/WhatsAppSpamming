use std::{thread::sleep, time::Duration};

use enigo::{Enigo, MouseControllable};

pub fn show_screen_coords() {
    let enigo = Enigo::new();
    loop {
        let cursor_location: (i32, i32) = enigo.mouse_location();

        match cursor_location {
            (0..=5, 0..=5) => break,
            (_, _) => {
                println!("{:?}", cursor_location);
                sleep(Duration::from_millis(500));
            }
        };
    }
}
