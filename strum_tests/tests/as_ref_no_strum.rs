#[macro_use]
extern crate strum_macros;

#[derive(Debug, Eq, PartialEq, AsRefStr)]
enum Color {
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(default = "true")]
    Green(String),
}

#[test]
fn as_red_str() {
    assert_eq!("RedRed", (Color::Red).as_ref());
}

#[test]
fn as_blue_str() {
    assert_eq!("blue", (Color::Blue { hue: 0 }).as_ref());
}

#[test]
fn as_yellow_str() {
    assert_eq!("yellow", (Color::Yellow).as_ref());
}

#[test]
fn as_green_str() {
    assert_eq!("Green", (Color::Green(String::default())).as_ref());
}
