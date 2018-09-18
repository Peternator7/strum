extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
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
    assert_eq!(String::from("blue"), format!("{}", Color::Blue { hue: 0 }));
}

#[test]
fn to_yellow_string() {
    assert_eq!(String::from("yellow"), format!("{}", Color::Yellow));
}

#[test]
fn to_red_string() {
    assert_eq!(String::from("RedRed"), format!("{}", Color::Red));
}
