extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use strum::{IntoEnumIterator, EnumMessage};

#[derive(Debug,Eq,PartialEq,EnumString,EnumIter,EnumMessage)]
enum Color {
    #[strum(message="The color red")]
    Red,
    #[strum(message="Blue blue")]
    Blue { hue: usize },
    #[strum(serialize="y",serialize="yellow",message="This is the color yellow", detailed_message="This is the detailed message.")]
    Yellow,
    #[strum(default="true",message="greenies")]
    Green(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_basic() {
        assert_eq!(Color::Red, Color::from_str("Red").unwrap());
    }

    #[test]
    fn from_str_default() {
        assert_eq!(Ok(Color::Green(String::from("White"))),
                   Color::from_str("White"));
    }
}
