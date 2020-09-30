// You need to bring the ToString trait into scope to use it
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug)]
enum Color {
    #[strum(serialize = "redred")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
}

fn main() {
    // uses the serialize string for Display
    let red = Color::Red;
    assert_eq!(String::from("redred"), red.to_string());
    // by default the variants Name
    let yellow = Color::Yellow;
    assert_eq!(String::from("Yellow"), yellow.to_string());
    // or for string formatting
    println!(
        "blue: {} green: {}",
        Color::Blue(10),
        Color::Green { range: 42 }
    );
}
