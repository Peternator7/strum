extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use std::string::ToString;

#[derive(Debug,Eq,PartialEq,EnumString,ToString)]
enum Color {
    #[strum(to_string="RedRed")]
    Red,
    #[strum(serialize="b", to_string="blue")]
    Blue { hue: usize },
    #[strum(serialize="y", serialize="yellow")]
    Yellow,
    #[strum(default="true")]
    Green(String),
}

#[test]
fn color_simple() {
    assert_eq!(Color::Red, Color::from_str("RedRed").unwrap());
}

#[test]
fn to_blue_string() {
    assert_eq!(String::from("blue"),
               (Color::Blue { hue: 0 }).to_string().as_ref());
}

#[test]
fn to_yellow_string() {
    assert_eq!(String::from("yellow"), (Color::Yellow).to_string().as_ref());
}

#[test]
fn to_red_string() {
    assert_eq!(String::from("RedRed"), (Color::Red).to_string());
    assert_eq!(Color::Red,
               Color::from_str((Color::Red).to_string().as_ref()).unwrap());
}
