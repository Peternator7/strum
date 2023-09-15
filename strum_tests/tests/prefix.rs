use strum::{Display, EnumString};

#[allow(dead_code)]
#[derive(Debug, EnumString, Display)]
#[strum(prefix = "colour/")]
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
fn prefix_redred() {
    assert_eq!(String::from("colour/RedRed"), (Color::Red).to_string());
}
