use strum::{Display, EnumString};
use strum_macros::AsRefStr;

#[allow(dead_code)]
#[derive(Debug, EnumString, Display, AsRefStr)]
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
    assert_eq!(("colour/RedRed"), (Color::Red).as_ref());
}

#[test]
fn prefix_green_default() {
    assert_eq!(
        String::from("green"),
        (Color::Green("green".into())).to_string()
    );

    assert_eq!(
        String::from("colour/Green"),
        (Color::Green("green".into())).as_ref()
    );
}
