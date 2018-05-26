extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;

use strum::AsStaticRef;

#[derive(Debug,Eq,PartialEq,EnumString,AsRefStr)]
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
fn as_red_str() {
    assert_eq!("RedRed",
               (Color::Red).as_ref());
    assert_eq!(Color::Red,
               Color::from_str((Color::Red).as_ref()).unwrap());
    let _: &'static str = Color::Red.as_static();
}

#[test]
fn as_blue_str() {
    assert_eq!("blue",
               (Color::Blue { hue: 0 }).as_ref());
    let _: &'static str = (Color::Blue { hue: 0 }).as_static();
}

#[test]
fn as_yellow_str() {
    assert_eq!("yellow", (Color::Yellow).as_ref());
    let _: &'static str = Color::Yellow.as_static();
}

#[test]
fn as_green_str() {
    assert_eq!("Green", (Color::Green(String::default())).as_ref());
    let _: &'static str = Color::Green(String::default()).as_static();
}
