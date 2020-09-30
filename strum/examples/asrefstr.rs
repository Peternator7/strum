// You need to bring the AsRef trait into scope to use it
use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
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
    assert_eq!("redred", red.as_ref());
    // by default the variants Name
    let yellow = Color::Yellow;
    assert_eq!("Yellow", yellow.as_ref());
    // or for string formatting
    println!(
        "blue: {} green: {}",
        Color::Blue(10).as_ref(),
        Color::Green { range: 42 }.as_ref()
    );
}
