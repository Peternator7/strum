use strum::{Display, EnumString};
use strum_macros::AsRefStr;

#[allow(dead_code)]
#[derive(Debug, EnumString, Display, AsRefStr)]
#[strum(suffix = ".color")]
enum Color {
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(default)]
    Green(String),
}

#[test]
fn suffix_redred() {
    let c = Color::Red;
    assert_eq!(String::from("RedRed.color"), c.to_string());
    assert_eq!("RedRed.color", c.as_ref());
}

#[test]
fn suffix_blue() {
    let c = Color::Blue { hue: 10 };
    assert_eq!(String::from("blue.color"), c.to_string());
    assert_eq!("blue.color", c.as_ref());
}

#[test]
fn suffix_yellow() {
    let c = Color::Yellow;
    assert_eq!(String::from("yellow.color"), c.to_string());
    assert_eq!("yellow.color", c.as_ref());
}

#[test]
fn suffix_green_default() {
    let c = Color::Green("basic-green".into());
    assert_eq!(String::from("basic-green"), c.to_string());
    assert_eq!("Green.color", c.as_ref());
}
