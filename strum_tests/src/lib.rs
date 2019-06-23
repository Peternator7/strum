#![allow(unused_imports)]
extern crate strum;
#[macro_use]
extern crate strum_macros;
use strum::*;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumString, ToString, EnumCount, EnumDiscriminants)]
pub enum Color {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled = "true")]
    Green(String),
}
