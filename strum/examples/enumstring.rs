//! Example howto use EnumString

use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Color {
    Red,
    // The Default value will be inserted into range if we match "Green".
    Green {
        range: usize,
    },

    // We can match on multiple different patterns.
    #[strum(serialize = "blue", serialize = "b")]
    Blue(usize),

    // Notice that we can disable certain variants from being found
    #[strum(disabled)]
    Yellow,
}

/*
//The generated code will look like:
impl std::str::FromStr for Color {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> ::std::result::Result<Color, Self::Err> {
        match s {
            "Red" => ::std::result::Result::Ok(Color::Red),
            "Green" => ::std::result::Result::Ok(Color::Green { range:Default::default() }),
            "blue" | "b" => ::std::result::Result::Ok(Color::Blue(Default::default())),
            _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound),
        }
    }
}
*/

fn main() {
    // simple from string
    let color_variant = Color::from_str("Red").unwrap();
    assert_eq!(Color::Red, color_variant);
    // short version works too
    let color_variant = Color::from_str("b").unwrap();
    assert_eq!(Color::Blue(0), color_variant);
    // was disabled for parsing = returns parse-error
    let color_variant = Color::from_str("Yellow");
    assert!(color_variant.is_err());
    // however the variant is still normally usable
    println!("{:?}", Color::Yellow);
}
