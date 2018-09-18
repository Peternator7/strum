extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Eq, PartialEq, EnumString, ToString)]
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
fn to_blue_string() {
    assert_eq!(
        String::from("blue"),
        (Color::Blue { hue: 0 }).to_string().as_ref()
    );
}

#[test]
fn to_yellow_string() {
    assert_eq!(String::from("yellow"), (Color::Yellow).to_string().as_ref());
}

#[test]
fn to_red_string() {
    assert_eq!(String::from("RedRed"), (Color::Red).to_string());
    assert_eq!(
        Color::Red,
        Color::from_str((Color::Red).to_string().as_ref()).unwrap()
    );
}

#[derive(Debug, Eq, PartialEq, ToString)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "bright")]
    BrightWhite,
}

#[test]
fn brightness_to_string() {
    assert_eq!(
        String::from("dark_black"),
        Brightness::DarkBlack.to_string().as_ref()
    );
    assert_eq!(
        String::from("dim"),
        Brightness::Dim { glow: 0 }.to_string().as_ref()
    );
    assert_eq!(
        String::from("bright"),
        Brightness::BrightWhite.to_string().as_ref()
    );
}
