use enigo::*;
use std::{thread, time};

fn main() {

    println_mouse_possiont();

    //enigo.mouse_move_to(500, 200);
    //enigo.mouse_click(MouseButton::Left);
    //enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}");
}

fn println_mouse_possiont() {
    let mut enigo = Enigo::new();
    let one_sec = time::Duration::from_millis(1000);
    let max_duration = time::Duration::from_millis(10000);
    let now = time::Instant::now();

    while time::Instant::now() < now + max_duration {
        let location = enigo.mouse_location();
        println!("{x}, {y}", x= location.0, y= location.1);
        thread::sleep(one_sec);
    }
}
